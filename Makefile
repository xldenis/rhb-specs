.PHONY: clean proofs reset-session build-lib build

clean: reset-session
	cargo clean

reset-session:
	rm -rf proofs

build:
	cargo creusot --features=contracts
	cp target/debug/rhb_specs.mlcfg proofs.mlcfg

proofs: build
