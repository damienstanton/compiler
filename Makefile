default: test release

.PHONY: test
test:
	@cargo test -- --nocapture

.PHONY: release
release:
	@cargo build --release

.PHONY: doc
doc:
	@cargo doc --no-deps --open
