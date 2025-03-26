//! PookaToken ERC-20 implementation for Arbitrum Stylus
use stylus_sdk::{prelude::*, storage::StorageAddress};

// Define the storage struct using Stylus macros
#[solidity_storage]
#[derive(Default)]
struct PookaToken {
    owner: StorageAddress,
    hook: StorageAddress,
    balances: StorageMap<Address, U256>,
    allowances: StorageMap<(Address, Address), U256>,
    total_supply: StorageU256,
}

#[external]
impl PookaToken {
    /// Initializes the contract with initial supply and hook address
    #[constructor]
    fn new(initial_supply: U256, hook_address: Address) -> Result<(), Vec<u8>> {
        let owner = msg::sender();
        self.owner.set(owner);
        self.hook.set(hook_address);
        self._mint(owner, initial_supply)
    }

    /// ERC-20 Token name
    #[view]
    fn name() -> Result<String, Vec<u8>> {
        Ok("Pooka Token".into())
    }

    /// ERC-20 Token symbol
    #[view]
    fn symbol() -> Result<String, Vec<u8>> {
        Ok("POOKA".into())
    }

    /// ERC-20 Total supply
    #[view]
    fn total_supply(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_supply.get())
    }

    /// ERC-20 Balance of address
    #[view]
    fn balance_of(&self, account: Address) -> Result<U256, Vec<u8>> {
        Ok(self.balances.get(account).unwrap_or_default())
    }

    /// ERC-20 Allowance between owner and spender
    #[view]
    fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Vec<u8>> {
        Ok(self.allowances.get((owner, spender)).unwrap_or_default())
    }

    /// Mint new tokens (owner only)
    #[external]
    fn mint(&mut self, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        self._mint(to, amount)
    }

    /// Burn tokens (owner only)
    #[external]
    fn burn(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        self._burn(msg::sender(), amount)
    }

    /// Hook-controlled transfer
    #[external]
    fn transfer(&mut self, from: Address, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        self.only_hook()?;
        self._transfer(from, to, amount)
    }

    /// Hook-controlled approval
    #[external]
    fn approve(&mut self, owner: Address, spender: Address, amount: U256) -> Result<(), Vec<u8>> {
        self.only_hook()?;
        self._approve(owner, spender, amount)
    }

    /// Standard ERC-20 Transfer
    #[external]
    fn transfer_erc20(&mut self, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        self._transfer(msg::sender(), to, amount)
    }

    /// Standard ERC-20 Transfer From
    #[external]
    fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<(), Vec<u8>> {
        let sender = msg::sender();
        let allowance = self.allowances.get((from, sender)).unwrap_or_default();
        
        if allowance < amount {
            return Err(b"Insufficient allowance".to_vec());
        }

        self.allowances.insert((from, sender)), allowance - amount);
        self._transfer(from, to, amount)
    }

    /// Standard ERC-20 Approve
    #[external]
    fn approve_erc20(&mut self, spender: Address, amount: U256) -> Result<(), Vec<u8>> {
        self._approve(msg::sender(), spender, amount)
    }
}

// Internal implementation
impl PookaToken {
    /// Authorization: Contract owner
    fn only_owner(&self) -> Result<(), Vec<u8>> {
        if msg::sender() == self.owner.get() {
            Ok(())
        } else {
            Err(b"Unauthorized: Owner only".to_vec())
        }
    }

    /// Authorization: Registered hook
    fn only_hook(&self) -> Result<(), Vec<u8>> {
        if msg::sender() == self.hook.get() {
            Ok(())
        } else {
            Err(b"Unauthorized: Hook only".to_vec())
        }
    }

    /// Internal mint implementation
    fn _mint(&mut self, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        let new_supply = self.total_supply.get().checked_add(amount).ok_or(b"Overflow")?;
        self.total_supply.set(new_supply);
        
        let new_balance = self.balance_of(to)?.checked_add(amount).ok_or(b"Overflow")?;
        self.balances.insert(to, new_balance);
        Ok(())
    }

    /// Internal burn implementation
    fn _burn(&mut self, from: Address, amount: U256) -> Result<(), Vec<u8>> {
        let balance = self.balance_of(from)?;
        if balance < amount {
            return Err(b"Insufficient balance".to_vec());
        }

        let new_supply = self.total_supply.get().checked_sub(amount).ok_or(b"Underflow")?;
        self.total_supply.set(new_supply);
        
        self.balances.insert(from, balance - amount);
        Ok(())
    }

    /// Internal transfer implementation
    fn _transfer(&mut self, from: Address, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        let from_balance = self.balance_of(from)?;
        if from_balance < amount {
            return Err(b"Insufficient balance".to_vec());
        }

        let to_balance = self.balance_of(to)?;
        self.balances.insert(from, from_balance - amount);
        self.balances.insert(to, to_balance + amount);
        Ok(())
    }

    /// Internal approve implementation
    fn _approve(&mut self, owner: Address, spender: Address, amount: U256) -> Result<(), Vec<u8>> {
        self.allowances.insert((owner, spender)), amount);
        Ok(())
    }
}