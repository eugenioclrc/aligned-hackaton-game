// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {LeaderBoard} from "./LeaderBoard.sol";

contract Verifier {
    LeaderBoard public immutable leaderboard = new LeaderBoard();

    address public immutable alignedServiceManager;
    address public immutable paymentServiceAddr;

    // @notice generated with `aligned get-vk-commitment --verification_key_file  elf/riscv32im-succinct-zkvm-elf  --proving_system SP1`
    bytes32 public immutable elfCommitment = 0xf562e7f8b6744459962a7c5d64fc1888b1bc46f9172e59978d346c525e08b67a;

    error InvalidElf(bytes32 submittedElf); // c6d95066

    struct Map {
        uint256 rows;
        uint256 cols;
        uint256 playerCol;
        uint256 playerRow;
        bytes mapData;
    }

    event MapMinted(uint256 tokenId, address indexed player, Map map, uint256 steps);

    // no need
    // map to check if proof has already been submitted
    // mapping(bytes32 => bool) public mintedProofs;

    constructor(address _alignedServiceManager, address _paymentServiceAddr) {
        alignedServiceManager = _alignedServiceManager;
        paymentServiceAddr = _paymentServiceAddr;
    }

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex,
        bytes memory pubInputBytes
    ) external returns (uint256) {
        if (elfCommitment != provingSystemAuxDataCommitment) {
            revert InvalidElf(provingSystemAuxDataCommitment);
        }

        require(address(proofGeneratorAddr) == msg.sender, "proofGeneratorAddr does not match");

        (uint256 stepsNumber, uint256 rows, uint256 cols, uint256 playerCol, uint256 playerRow, bytes memory mapData) =
            abi.decode(pubInputBytes, (uint256, uint256, uint256, uint256, uint256, bytes));

        uint256 tokenId = uint256(keccak256(abi.encode(rows, cols, playerCol, playerRow, mapData)));

        /*
        check is being done on token erc1155 mint function
        bytes32 fullHash = keccak256(
            abi.encodePacked(
                proofCommitment,
                pubInputCommitment,
                provingSystemAuxDataCommitment,
                proofGeneratorAddr
            )
        );
        require(!mintedProofs[fullHash], "proof already minted");
        mintedProofs[fullHash] = true;
        */

        (bool callWasSuccessfull, bytes memory proofIsIncluded) = alignedServiceManager.staticcall(
            abi.encodeWithSignature(
                "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,address)",
                proofCommitment,
                pubInputCommitment,
                provingSystemAuxDataCommitment,
                proofGeneratorAddr,
                batchMerkleRoot,
                merkleProof,
                verificationDataBatchIndex,
                paymentServiceAddr
            )
        );

        require(callWasSuccessfull, "static_call failed");

        bool proofIsIncludedBool = abi.decode(proofIsIncluded, (bool));
        require(proofIsIncludedBool, "proof not included in batch");

        // mintedProofs[fullHash] = true;

        leaderboard.mint(tokenId, msg.sender, stepsNumber);

        emit MapMinted(
            tokenId,
            msg.sender,
            Map({rows: rows, cols: cols, playerCol: playerCol, playerRow: playerRow, mapData: mapData}),
            stepsNumber
        );
    }
}
