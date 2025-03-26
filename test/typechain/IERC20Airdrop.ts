/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
  BaseContract,
  BigNumberish,
  BytesLike,
  FunctionFragment,
  Result,
  Interface,
  AddressLike,
  ContractRunner,
  ContractMethod,
  Listener,
} from "ethers";
import type {
  TypedContractEvent,
  TypedDeferredTopicFilter,
  TypedEventLog,
  TypedListener,
  TypedContractMethod,
} from "./common";

export interface IERC20AirdropInterface extends Interface {
  getFunction(
    nameOrSignature: "claim" | "restAirdrop" | "totalAirdrop"
  ): FunctionFragment;

  encodeFunctionData(
    functionFragment: "claim",
    values: [AddressLike, BigNumberish]
  ): string;
  encodeFunctionData(
    functionFragment: "restAirdrop",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "totalAirdrop",
    values?: undefined
  ): string;

  decodeFunctionResult(functionFragment: "claim", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "restAirdrop",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "totalAirdrop",
    data: BytesLike
  ): Result;
}

export interface IERC20Airdrop extends BaseContract {
  connect(runner?: ContractRunner | null): IERC20Airdrop;
  waitForDeployment(): Promise<this>;

  interface: IERC20AirdropInterface;

  queryFilter<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>;
  queryFilter<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>;

  on<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    listener: TypedListener<TCEvent>
  ): Promise<this>;
  on<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>;

  once<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    listener: TypedListener<TCEvent>
  ): Promise<this>;
  once<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>;

  listeners<TCEvent extends TypedContractEvent>(
    event: TCEvent
  ): Promise<Array<TypedListener<TCEvent>>>;
  listeners(eventName?: string): Promise<Array<Listener>>;
  removeAllListeners<TCEvent extends TypedContractEvent>(
    event?: TCEvent
  ): Promise<this>;

  claim: TypedContractMethod<
    [receiver: AddressLike, amount: BigNumberish],
    [void],
    "nonpayable"
  >;

  restAirdrop: TypedContractMethod<[], [bigint], "nonpayable">;

  totalAirdrop: TypedContractMethod<[], [bigint], "view">;

  getFunction<T extends ContractMethod = ContractMethod>(
    key: string | FunctionFragment
  ): T;

  getFunction(
    nameOrSignature: "claim"
  ): TypedContractMethod<
    [receiver: AddressLike, amount: BigNumberish],
    [void],
    "nonpayable"
  >;
  getFunction(
    nameOrSignature: "restAirdrop"
  ): TypedContractMethod<[], [bigint], "nonpayable">;
  getFunction(
    nameOrSignature: "totalAirdrop"
  ): TypedContractMethod<[], [bigint], "view">;

  filters: {};
}
