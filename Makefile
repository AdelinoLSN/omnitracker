.PHONY: test coverage

test:
	cargo test --manifest-path apps/api/Cargo.toml -- --test-threads=1

coverage:
	cargo llvm-cov --manifest-path apps/api/Cargo.toml -- --test-threads=1
