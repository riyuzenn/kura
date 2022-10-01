
TARGET=/target/debug

.PHONY: help, copyright

help:
	$(info Kura Makefile)
	$(info )
	$(info Consider to use 'cargo' instead)
	$(info )
	
	@grep '^[[:alnum:]_-]*:.* ##' $(MAKEFILE_LIST) \
		| sort | awk 'BEGIN {FS=":.* ## "}; {printf "%-25s %s\n", $$1, $$2};'

install: # Build and install the app and store in the local path
	@cargo build --release
	@cp target/release/kura ~/.local/bin/

run: # Run the app package
	@RUST_BACKTRACE=full RUST_LOG=info cargo run --release -p app


clean:
	@cargo clean

test:
	@RUST_BACKTRACE=full RUST_LOG=info cargo test

check-setup:
	@type rustup >/dev/null 2>&1 || (echo "Install rustup first. To install, run Run 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh'" >&2 ; exit 1)
	@type rustc >/dev/null 2>&1 || (echo "Install rustc first. To install, run Run 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh'" >&2 ; exit 1)

lint:
	@cargo fix
	@cargo clippy --fix -Z unstable-options
	@cargo clippy --all-targets --all-features -- -D warnings

style:
	@cargo fmt

doc:
	@cargo doc --target-dir docs

copyright: # Add copyright information to each rust file
	@find . -iname "*.rs" -exec bash -c "if ! grep -q Copyright "{}"; then cat copyright {} > {}.new && mv {}.new {} ; fi" \; 
	@printf "Copyright notice added.\n"
