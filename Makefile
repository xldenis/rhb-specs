.PHONY: clean proofs reset-session build-lib build

clean: reset-session
	cargo clean

reset-session:
	rm -r proofs

build-lib:
	CREUSOT_CONTINUE=1 cargo creusot --features=contracts > /dev/null
	cargo clean -p rhb-specs

build: build-lib
	CREUSOT_CONTINUE=1 cargo creusot --features=contracts > proofs.mlcfg

proofs: build
