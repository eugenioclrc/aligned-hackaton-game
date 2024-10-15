# Variables for directories
PROG_DIR = program
PROVE_DIR = prove
GAME_DIR = script

# Build targets
all: build-elf build-prover build-game

.PHONY: all build-elf build-prover build-game

build-elf:
	@echo "Building ELF files"
	cd $(PROG_DIR) && cargo prove build

build-prover:
	@echo "Building PROVER files"
	cd $(PROVE_DIR) && cargo build --bin sokoban --release

build-game:
	@echo "Building end user GAME"
	cd $(GAME_DIR) && cargo build --release

clean:
	@echo "Cleaning all builds"
	cd $(PROG_DIR) && cargo clean
	cd $(PROVE_DIR) && cargo clean
	cd $(GAME_DIR) && cargo clean