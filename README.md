# aligned-hackaton-game

## Description POC
This is a POC to do a basic game like sokoban [REF](https://en.wikipedia.org/wiki/Sokoban) using aligned smart contracts.

## Build and run
To build and run the game you need to have `cargo` installed. You can install it using `rustup` [REF](https://rustup.rs/).

To build the game you can use the following command:
```bash
make all
```

This will build the game and the prove tool.

## Run the game

To run the game you can use the following command:
```bash
./script/target/release/game
```

After you win the game you will see get a valid path to prove the game.
```json
{"path":"0x0894cfd894f5a000000000000000000000000000000000000000000000000000","length":26}
```

## Prove the game
This will build the run the game using your path to prove the game and build the proves.
```bash
./prove/target/release/sokoban --prove --moves '{"path":"0x08943fd894f5a000000000000000000000000000000000000000000000000000","length":26}'
```

## Submit prove
Use

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
    --proving_system SP1 \
    --proof prove/proof.bin \
    --vm_program program/elf/riscv32im-succinct-zkvm-elf \
    --aligned_verification_data_path ./aligned_verification_data \
    --batcher_url wss://batcher.alignedlayer.com \
    --network holesky \
    --rpc_url https://ethereum-holesky-rpc.publicnode.com

2024-10-15T19:10:11Z WARN  aligned] Missing keystore used for payment. This proof will not be included if sent to Eth Mainnet
[2024-10-15T19:10:13Z INFO  aligned] Submitting proofs to the Aligned batcher...
[2024-10-15T19:10:17Z ERROR aligned_sdk::communication::messaging] Invalid Proof!
[2024-10-15T19:10:17Z ERROR aligned] Submitted proof is invalid
```