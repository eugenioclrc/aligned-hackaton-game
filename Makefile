# Variables for directories
PROG_DIR = program
PROVE_DIR = prove
GAME_DIR = script

# Build targets
all: build-elf build-prover build-game

.PHONY: all build-elf build-prover build-game

build-elf:
	@echo "Building ELF files"
	cd $(PROG_DIR) && cargo prove build && aligned get-vk-commitment --verification_key_file  elf/riscv32im-succinct-zkvm-elf  --proving_system SP1 2> elf/commitment

build-prover:
	@echo "Building PROVER files"
	cd $(PROVE_DIR) && cargo build --bin sokoban --release

build-game:
	@echo "Building end user GAME"
	cd $(GAME_DIR) && cargo build --release

proof:
	@echo "Building PROVE files"
	cd $(PROVE_DIR) && ./target/release/sokoban --prove --moves '{"path":"0x08943fd894f5a000000000000000000000000000000000000000000000000000","length":26,"map":"todo"}'

clean:
	@echo "Cleaning all builds"
	cd $(PROG_DIR) && cargo clean
	cd $(PROVE_DIR) && cargo clean
	cd $(GAME_DIR) && cargo clean