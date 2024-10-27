# aligned-hackaton-game

This is a game in which you feature a worker in align that needs to move the proof boxes to the storage locations. The worker can push boxes but can't pull them. The worker can only push one box at a time and can't push boxes into walls or other boxes.

# ZK Sokoban - Table of Contents

1. [Introduction](#aligned-hackaton-game)
   - [Project Overview](#aligned-hackaton-game)
   - [Game Mechanics](#aligned-hackaton-game)

2. [Technical Implementation](#technical-implementation)
   - [Architecture](#architecture)
     - [Smart Contract Layer](#architecture)
     - [Game Engine](#architecture)
     - [ZK Proof Generation](#architecture)
   - [Technical Description (POC)](#description-poc)
     - [Map Data Structure](#description-poc)
     - [NFT Minting Mechanics](#description-poc)
     - [Prover Data Structure](#description-poc)
     - [Public Contract Inputs](#description-poc)

3. [Development Guide](#build-and-run)
   - [Prerequisites](#build-and-run)
   - [Build Instructions](#build-and-run)
   - [Running the Game](#run-the-game-and-submit-score)
   - [Proof Generation](#prove-the-game)

4. [Project Components](#project-structure)
   - [Browser Game Frontend](#browser-game)
   - [Game Logic Core](#game-logic)
   - [SP1 Program](#program)
   - [Proof Generation Tool](#prove)
   - [Leaderboard Contract](#leaderboard-contract)
   - [Level Generation Utility](#levelmakerjs)

5. [Future Development](#roadmap)
   - [Phase 1 (Current Features)](#phase-1-current)
   - [Phase 2 (Planned Features)](#phase-2-next-steps)

---

*Note: Each section header is a clickable link that jumps to the corresponding content in the documentation.*

## Technical Implementation

### Architecture
The project consists of several key components:

1. **Smart Contract Layer**
   - Handles proof verification using Aligned
   - Manages leaderboard and NFT minting
   - Stores puzzle metadata and solutions

2. **Game Engine**
   - Built with Phaser.js for smooth browser-based gameplay
   - Implements core Sokoban mechanics
   - Generates solution paths for ZK proof creation

3. **ZK Proof Generation**
   - Uses SP1 for generating zero-knowledge proofs
   - Validates solution correctness and move count
   - Ensures tamper-proof solution verification

### Description POC
This is a POC to do a basic game like sokoban [REF](https://en.wikipedia.org/wiki/Sokoban) using Sp1 and Aligned.

Every map is represented by this data:
```json
{
    "map": "hexadecimal that represent the tiles in de map, 00 empty, 10 wall, 01 box, 11 Target to put the box",
    "rows": "u32 width of the map",
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
    "rows": "u32 width of the map",
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
    "rows": "u32 width of the map",
    "cols": "u32 height of the map",
    "player_col": "u32 player start pos col",
    "player_row": "u32 player start pos row"
}
```

## Build and run
To build and run the game you need to have `cargo` installed. You can install it using `rustup` [REF](https://rustup.rs/).

To build the game you can use the following command:
```shell
$ make all
```

This will build the game and the prove tool.

## Run the game and submit score

To run the game you can use the web version on [https://aligned-hackaton-game.vercel.app/](https://aligned-hackaton-game.vercel.app/)

After you win the game you will see get a command to prove the game and submit you proove.


## Prove the game
This will build the run the game using your path to prove the game and build the proves. Example;
```bash
./prove/target/release/sokoban --data '{"rows":6,"cols":7,"map":"aaaa002844a222bc0aaaa0","player_row":2,"player_col":1,"path":"3FD89894F4F5A","length":26}' --keystore-path ~/.foundry/keystores/keystore0
```

## Project structure

### [`/browser-game`](./browser-game)

This is the frontend of the game. It uses Phaser to render the game.

### [`/game-logic`](./game-logic)

This is the game logic of the sokoban, imported in the SP1 program.

### [`/program`](./program)

This is the SP1 program that will be used to prove the game.

### [`/prove`](./prove)

This is the prove tool that will be used to prove the game and submit to align and then mint the nft.

### [`/leaderboard-contract`](./leaderboard-contract)

This is the contract that will be used to check proof in aligned, store the leaderboard of the game and mint the nfts.

### [`./LevelMaker.js`](./LevelMaker.js)

This is a simple node script to generate levels for the game.

## Roadmap

### Phase 1 (Current)
- ✅ Core gameplay implementation
- ✅ Basic ZK proof generation
- ✅ Smart contract deployment
- ✅ Web interface

### Phase 2 (Next Steps)
- 🔄 Enhanced puzzle generation system
- 🔄 Enhanced & improve front end
- 🔄 Community puzzle creation tools
- 🔄 Tournament system
