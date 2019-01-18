.PHONY: test
test:
	@cargo test -- --nocapture

.PHONY: release
release:
	@cargo build --release

.PHONY: docs
docs:
	@cargo doc --no-deps --open
