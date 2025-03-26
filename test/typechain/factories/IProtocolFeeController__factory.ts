/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Contract, Interface, type ContractRunner } from "ethers";
import type {
  IProtocolFeeController,
  IProtocolFeeControllerInterface,
} from "../IProtocolFeeController";

const _abi = [
  {
    type: "function",
    name: "protocolFeeForPool",
    inputs: [
      {
        name: "poolKey",
        type: "tuple",
        internalType: "struct PoolKey",
        components: [
          {
            name: "currency0",
            type: "address",
            internalType: "Currency",
          },
          {
            name: "currency1",
            type: "address",
            internalType: "Currency",
          },
          {
            name: "fee",
            type: "uint24",
            internalType: "uint24",
          },
          {
            name: "tickSpacing",
            type: "int24",
            internalType: "int24",
          },
          {
            name: "hooks",
            type: "address",
            internalType: "contract IHooks",
          },
        ],
      },
    ],
    outputs: [
      {
        name: "",
        type: "uint16",
        internalType: "uint16",
      },
    ],
    stateMutability: "view",
  },
] as const;

export class IProtocolFeeController__factory {
  static readonly abi = _abi;
  static createInterface(): IProtocolFeeControllerInterface {
    return new Interface(_abi) as IProtocolFeeControllerInterface;
  }
  static connect(
    address: string,
    runner?: ContractRunner | null
  ): IProtocolFeeController {
    return new Contract(
      address,
      _abi,
      runner
    ) as unknown as IProtocolFeeController;
  }
}
