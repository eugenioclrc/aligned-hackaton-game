# Variables for directories
PROG_DIR = program
PROVE_DIR = prove

# Build targets
all: build-elf build-prover build-game

.PHONY: all build-elf build-prover build-game

build-elf:
	@echo "Building ELF files"
	cd $(PROG_DIR) && cargo prove build && aligned get-vk-commitment --verification_key_file  elf/riscv32im-succinct-zkvm-elf  --proving_system SP1 2> elf/commitment

build-prover:
	@echo "Building PROVER files"
	cd $(PROVE_DIR) && cargo build --release

proof:
	@echo "Building PROVE files"
	./prove/target/release/zokoban --data '{"rows":6,"cols":7,"map":"aaaa002844a222bc0aaaa0","player_row":2,"player_col":1,"path":"3FD89894F4F5A","length":26}' --keystore-path ~/.foundry/keystores/keystore0

clean:
	@echo "Cleaning all builds"
	cd $(PROG_DIR) && cargo clean
	cd $(PROVE_DIR) && cargo clean
