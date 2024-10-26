// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Verifier} from "../src/Verifier.sol";

contract VerifierTest is Test {
    Verifier public verifier;

    address ALIGNED_SERVICE_MANAGER_ADDRESS = 0x58F280BeBE9B34c9939C3C39e0890C81f163B623;
    address BATCHER_PAYMENT_SERVICE_ADDRESS = 0x815aeCA64a974297942D2Bbf034ABEe22a38A003;

    address user = 0x889558Ea3C7b58b544EB17a6Fc04044547837a77;

    function setUp() public {
        verifier = new Verifier(ALIGNED_SERVICE_MANAGER_ADDRESS, BATCHER_PAYMENT_SERVICE_ADDRESS);
        vm.label(user, "USER");
    }

    function test_mint() public {
        bytes32 proofCommitment = 0x61460a4f12f978dff7fa3e50910565aea7a580d69c616db97e3cd1410cde4bd5;
        bytes32 pubInputCommitment = 0xdd509e93688fb1126cc314fe67d90ba7c751bcd38ef704c91c72ee5139598f5b;
        bytes32 provingSystemAuxDataCommitment = 0xf562e7f8b6744459962a7c5d64fc1888b1bc46f9172e59978d346c525e08b67a;
        bytes20 proofGeneratorAddr = bytes20(user);
        bytes32 batchMerkleRoot = 0x100f0843c186a1b951ffc246b1458a9c37de1675f90afa8391fcc8c0a6a31c9c;
        bytes memory merkleProof =
            hex"c4f2ae5c75f70f0a4b009a34668099ac97c61db255da593534fbf62e6187dff11dcfac2bfec3a65149304173b9ecbb974b49a19ab5cc4a24488a6dedea2e8ebef5b7c221f16554b3e21baac2da75518f19a69bf5dd817535973ea099be573bb9";
        uint256 verificationDataBatchIndex = 0;
        bytes memory pubInputBytes =
            hex"000000000000000000000000000000000000000000000000000000000000001c000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000070000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000baaaa002844a222bc0aaaa0000000000000000000000000000000000000000000";

        bytes4 selector =
            bytes4(keccak256("verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,address)"));
        vm.mockCall(ALIGNED_SERVICE_MANAGER_ADDRESS, abi.encodeWithSelector(selector), abi.encode(true));

        vm.prank(user);
        verifier.verifyBatchInclusion(
            proofCommitment,
            pubInputCommitment,
            provingSystemAuxDataCommitment,
            proofGeneratorAddr,
            batchMerkleRoot,
            merkleProof,
            verificationDataBatchIndex,
            pubInputBytes
        );
    }
}
