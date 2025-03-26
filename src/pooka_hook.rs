//! PookaValuationHook for Uniswap V4 on Arbitrum Stylus

use stylus_sdk::{
  prelude::*,
  storage::{StorageAddress, StorageU256, StorageMap, StorageVec},
  console,
};
use uniswap_v4_sdk::{
  types::{PoolKey, SwapParams, BalanceDelta, Currency},
  hooks::{BaseHook, HookPermissions, BeforeSwapDelta},
  pool_manager::IPoolManager,
  tick_math::MIN_SQRT_PRICE,
  utils::LiquidityAmounts
};

#[solidity_storage]
#[derive(Default)]
struct PookaValuationHook {
  owner: StorageAddress,
  pool_manager: StorageAddress,
  dai: StorageAddress,
  pooka: StorageAddress,
  total_pooka_swapped: StorageU256,
  external_warchest: StorageU256,
  user_positions: StorageMap<Address, StorageVec<Position>>,
}

#[derive(Clone, StorageType)]
struct Position {
  pooka_amount: U256,
}

const INITIAL_FEE: u24 = 3000; // 0.3%
const FEE_REDUCTION_PER_STEP: u24 = 150; // 5% of 3000
const VOLUME_STEP: U256 = U256::from(1_000) * U256::exp10(18);

#[external]
impl PookaValuationHook {
  #[constructor]
  fn new(pool_manager: Address, dai: Address, pooka: Address) -> Result<(), Vec<u8>> {
      self.owner.set(msg::sender());
      self.pool_manager.set(pool_manager);
      self.dai.set(dai);
      self.pooka.set(pooka);
      Ok(())
  }

  #[view]
  fn get_hook_permissions(&self) -> HookPermissions {
      HookPermissions {
          before_initialize: false,
          after_initialize: false,
          before_add_liquidity: false,
          after_add_liquidity: false,
          before_remove_liquidity: false,
          after_remove_liquidity: false,
          before_swap: true,
          after_swap: true,
          before_donate: false,
          after_donate: false,
          ..HookPermissions::default()
      }
  }

  #[view]
  fn get_user_fee(&self, user: Address) -> Result<u24, Vec<u8>> {
      let positions = self.user_positions.get(user).unwrap_or_default();

      if positions.is_empty() {
          return Ok(INITIAL_FEE);
      }

      let pooka_amount = positions[0].pooka_amount;
      let mut steps = pooka_amount / VOLUME_STEP;
      steps = steps.min(U256::from(10)); // Max 10 steps

      let fee_reduction = steps.as_u24() * FEE_REDUCTION_PER_STEP;
      let calculated_fee = INITIAL_FEE.saturating_sub(fee_reduction);
      Ok(calculated_fee.max(INITIAL_FEE / 2))
  }

  #[external]
  fn adjust_warchest(
      &mut self,
      key: PoolKey,
      add_to_pool: bool,
      dai_amount: U256
  ) -> Result<(), Vec<u8>> {
      self.only_owner()?;
      
      let token0 = Currency::unwrap(key.currency0);
      let token1 = Currency::unwrap(key.currency1);
      let dai_addr = self.dai.get();
      let pooka_addr = self.pooka.get();

      require!(
          (token0 == dai_addr && token1 == pooka_addr) ||
          (token1 == dai_addr && token0 == pooka_addr),
          "Invalid pool pair"
      );

      let params = SwapParams {
          zero_for_one: add_to_pool,
          amount_specified: dai_amount.as_i256(),
          sqrt_price_limit_x96: if add_to_pool {
              MIN_SQRT_PRICE + 1
          } else {
              MIN_SQRT_PRICE - 1
          },
      };

      // Execute swap through PoolManager
      IPoolManager::swap(self.pool_manager.get(), key, params, vec![])?;

      if add_to_pool {
          self.external_warchest.set(self.external_warchest.get() - dai_amount);
      } else {
          self.external_warchest.set(self.external_warchest.get() + dai_amount);
      }

      Ok(())
  }

  #[external]
  fn before_swap(
      &mut self,
      _sender: Address,
      key: PoolKey,
      params: SwapParams,
      hook_data: Vec<u8>
  ) -> Result<(BeforeSwapDelta, u24), Vec<u8>> {
      let user: Address = abi_decode(hook_data)?;
      require!(user != Address::ZERO, "Invalid swap");

      let pooka_addr = self.pooka.get();
      let is_pooka_token0 = self.is_pooka_token0(key);

      // Update total swapped amount
      if (is_pooka_token0 && params.zero_for_one) || 
         (!is_pooka_token0 && !params.zero_for_one) {
          let amount = U256::from(-params.amount_specified);
          self.total_pooka_swapped.set(self.total_pooka_swapped.get() + amount);
      }

      if user == self.owner.get() {
          return Ok((BeforeSwapDelta::ZERO, 0));
      }

      let fee = self.get_user_fee(user)?;
      Ok((BeforeSwapDelta::ZERO, fee))
  }

  #[external]
  fn after_swap(
      &mut self,
      _sender: Address,
      key: PoolKey,
      _params: SwapParams,
      _delta: BalanceDelta,
      hook_data: Vec<u8>
  ) -> Result<(), Vec<u8>> {
      let user: Address = abi_decode(hook_data)?;
      require!(user != Address::ZERO, "Invalid swap");

      let pooka_balance = ERC20::balance_of(self.pooka.get(), user)?;
      if pooka_balance > U256::ZERO {
          self.update_position(user, true)?;
      }

      Ok(())
  }
}

impl PookaValuationHook {
  fn is_pooka_token0(&self, key: PoolKey) -> bool {
      Currency::unwrap(key.currency0) == self.pooka.get()
  }

  fn update_position(&mut self, user: Address, is_add: bool) -> Result<(), Vec<u8>> {
      let pooka_balance = ERC20::balance_of(self.pooka.get(), user)?;
      let mut positions = self.user_positions.entry(user).or_default();

      if is_add {
          positions.push(Position { pooka_amount: pooka_balance });
      } else {
          for position in positions.iter_mut() {
              position.pooka_amount = pooka_balance;
          }
      }

      Ok(())
  }

  fn only_owner(&self) -> Result<(), Vec<u8>> {
      require!(msg::sender() == self.owner.get(), "Unauthorized");
      Ok(())
  }
}