// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { FHE, ebool, euint8, externalEuint8 } from "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract FHEIfThenElse is SepoliaConfig {
  euint8 private _a;
  euint8 private _b;
  euint8 private _max;

  function setA(externalEuint8 inputA, bytes calldata inputProof) external {
    _a = FHE.fromExternal(inputA, inputProof);
    FHE.allowThis(_a);
  }

  function setB(externalEuint8 inputB, bytes calldata inputProof) external {
    _b = FHE.fromExternal(inputB, inputProof);
    FHE.allowThis(_b);
  }

  function computeMax() external {
    ebool a_ge_b = FHE.ge(_a, _b);
    _max = FHE.select(a_ge_b, _a, _b);
    FHE.allowThis(_max);
    FHE.allow(_max, msg.sender);
  }

  function result() public view returns (euint8) {
    return _max;
  }
}
```

```ts
import { expect } from "chai";
import { ethers } from "hardhat";
import * as hre from "hardhat";
import type { Signers } from "../../types";
import { HardhatFhevmRuntimeEnvironment } from "@fhevm/hardhat-plugin";

async function deployFixture() {
  const factory = await ethers.getContractFactory("FHEIfThenElse");
  const contract = await factory.deploy();
  await contract.deployed();
  
  return contract;
}

describe("FHEIfThenElse", () => {
  let contract: any;
  let signers: Signers;
  
  before(async () => {
    if (!hre.fhevm.isMock) throw new Error("Tests must run on fhEVM mock");
    
    const ethSigners = await ethers.getSigners();
    signers = { owner: ethSigners[0], alice: ethSigners[1] };
    
   });

   beforeEach(async () => {
     contract = await deployFixture();
   });

   it("computes max correctly", async () => {
     const fhevm: HardhatFhevmRuntimeEnvironment = hre.fhevm;

     const aVal = 80;
     const bVal =123;

     const encryptedAInput= await fhevm.createEncryptedInput(contract.address, signers.alice.address).add8(aVal).encrypt();
     await contract.connect(signers.alice).setA(encryptedAInput.handles[0], encryptedAInput.inputProof);

     const encryptedBInput=await fhevm.createEncryptedInput(contract.address,signers.alice.address).add8(bVal).encrypt();
     await contract.connect(signers.alice).setB(encryptedBInput.handles[0], encryptedBInput.inputProof);

     // Bob executes computeMax with proper permissions
     const bob= (await ethers.getSigners())[2];
     
     await contract.connect(bob).computeMax();

     // Retrieve and decrypt result
      const encResult=await contract.result();

      const decryptedMax=await fhevm.userDecryptEuint(
        hre.fhevm.FhevmType.euint8,
        encResult,
        contract.address,
        bob,
      );

      expect(decryptedMax).to.equal(aVal >= bVal ? aVal : bVal );
   });
});
