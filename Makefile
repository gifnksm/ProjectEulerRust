PKG=$(patsubst pkg/%,%,$(wildcard pkg/*))
RELEASE_PKG=$(patsubst %,release-%,$(PKG))
DEBUG_PKG=$(patsubst %,debug-%,$(PKG))
TEST_PKG=$(patsubst %,test-%,$(PKG))
BENCH_PKG=$(patsubst %,bench-%,$(PKG))


release: release-euler $(RELEASE_PKG)
release-euler: $(RELEASE_PKG)
	cargo build --release
release-%:
	cargo build --release -p $(patsubst release-%,%,$@)


debug: debug-euler $(DEBUG_PKG)
debug-euler: $(DEBUG_PKG)
	cargo build
debug-%:
	cargo build -p $(patsubst debug-%,%,$@)


test: test-euler $(TEST_PKG)
test-euler: $(TEST_PKG)
	cargo test
test-%:
	cargo test -p $(patsubst test-%,%,$@)


bench: bench-euler $(BENCH_PKG)
bench-euler: $(BENCH_PKG)
	cargo bench
bench-%:
	cargo bench -p $(patsubst bench-%,%,$@)


run-release: release
	cargo run --release --bin euler
run-debug: debug
	cargo run --bin euler
doc:
	cargo doc
clean:
	cargo clean
