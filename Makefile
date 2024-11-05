all:

.PHONY: disassemble
disassemble:
	cd cli; cargo run -p rbpf_cli -- -u disassembler --elf $(FILE)
