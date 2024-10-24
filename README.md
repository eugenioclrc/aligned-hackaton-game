# aligned-hackaton-game

This is a game in which you feature a worker in align that needs to move the proof boxes to the storage locations. The worker can push boxes but can't pull them. The worker can only push one box at a time and can't push boxes into walls or other boxes.

## Description POC
This is a POC to do a basic game like sokoban [REF](https://en.wikipedia.org/wiki/Sokoban) using Sp1 and Aligned.

Every map is represented by this data:
```json
{
    "map": "hexadecimal that represent the tiles in de map, 00 empty, 10 wall, 01 box, 11 Target to put the box",
    "row": "u32 width of the map",
    "cols": "u32 height of the map",
    "player_col": "u32 player start pos col",
    "player_row": "u32 player start pos row",
}
```

The keccak256 hash of the map is used to have the map id. If a player wins the game, and send a valid prove to demonstrate that has a valid path to solve the game, the player will be mint an nft with the map id and it will emit all the data to have the map data based on a hash.
The user can only mint the nft if the prove is valid, is the first on solving the map or if the map is solved he has to have a path with less moves than the previous one.

The proover will use the next data to prove the game:
```json
{
    "path": "hexadecimal that represent the path to solve the game",
    "length": "u32 length of the path",
    "map": "hexadecimal that represent the tiles in de map, 00 empty, 10 wall, 01 box, 11 Target to put the box",
    "row": "u32 width of the map",
    "cols": "u32 height of the map",
    "player_col": "u32 player start pos col",
    "player_row": "u32 player start pos row"
}
```

Finally the public inputs used on the contract are:
```json
{
    "length": "u32 length of the path",
    "map": "hexadecimal that represent the tiles in de map, 00 empty, 10 wall, 01 box, 11 Target to put the box",
    "row": "u32 width of the map",
    "cols": "u32 height of the map",
    "player_col": "u32 player start pos col",
    "player_row": "u32 player start pos row"
}
```

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
{
    "path":"0x0894cfd894f5a000000000000000000000000000000000000000000000000000",
    "length":26,
    "map":"todo"
}
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
    --keystore_path ~/.foundry/keystores/keystore0 \
    --rpc_url https://ethereum-holesky-rpc.publicnode.com

Please enter your keystore password:
[2024-10-21T20:01:09Z INFO  aligned] Submitting proofs to the Aligned batcher...
[2024-10-21T20:01:43Z INFO  aligned] Batch inclusion data written into ./aligned_verification_data/b194298f_0.json
[2024-10-21T20:01:43Z INFO  aligned] Proofs submitted to aligned. See the batch in the explorer:
[2024-10-21T20:01:43Z INFO  aligned] https://explorer.alignedlayer.com/batches/0xb194298fab098c1f3eef571cdc76e99974e668b7cf823562740bb1a5b3bf6e1e
```
