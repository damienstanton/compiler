.PHONY: test
test:
	@cargo test -- --nocapture

.PHONY: release
release:
	@cargo build --release
