// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Verifier} from "../src/Verifier.sol";
    
contract VerifierScript is Script {
    Verifier public verifier;

    address ALIGNED_SERVICE_MANAGER_ADDRESS = 0x58F280BeBE9B34c9939C3C39e0890C81f163B623;
    address BATCHER_PAYMENT_SERVICE_ADDRESS = 0x815aeCA64a974297942D2Bbf034ABEe22a38A003;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        verifier = new Verifier(ALIGNED_SERVICE_MANAGER_ADDRESS, BATCHER_PAYMENT_SERVICE_ADDRESS);

        vm.stopBroadcast();
    }
}
