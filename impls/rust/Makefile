
dist: mal

mal: stepA_mal
	cp $< $@

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS = Cargo.toml

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
