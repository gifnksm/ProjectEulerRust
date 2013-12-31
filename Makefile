PROB_SRC   = $(sort $(wildcard src/prob*.rs))
MOD_SRC    = src/euler/problem.rs
ALL_SRC    = $(wildcard src/*.rs) $(wildcard src/*/*.rs)

DEPEND_DIR=depend
DEBUG_RLIB_DIR=lib/debug
DEBUG_BIN_DIR=bin/debug

RELEASE_RLIB_DIR=lib/release
RELEASE_BIN_DIR=bin/release

TEST_BIN_DIR=bin/test

DEPEND=$(DEPEND_DIR)/all.mk

DEBUG_RUSTC_FLAGS   = -L $(DEBUG_RLIB_DIR)
RELEASE_RUSTC_FLAGS = --opt-level 3 -L $(RELEASE_RLIB_DIR)
TEST_RUSTC_FLAGS    = --test -L $(DEBUG_RLIB_DIR)

.PHONY: debug release test bench depend clean

debug:
release:
test:
bench:
depend: $(DEPEND)

clean:
	$(RM) -r $(MOD_SRC) $(DEPEND_DIR)/*
	$(RM) $(DEBUG_BIN_DIR)/* $(RELEASE_BIN_DIR)/* $(TEST_BIN_DIR)/*
	$(RM) $(DEBUG_RLIB_DIR)/*.rlib $(RELEASE_RLIB_DIR)/*.rlib

$(DEPEND): $(ALL_SRC) $(MOD_SRC) ./etc/mkdepend
	./etc/mkdepend > $@
$(MOD_SRC): $(PROB_SRC)
	./etc/genmod ./src > $@

ifneq "$(MAKECMDGOALS)" "clean"
include $(DEPEND)
endif

DEBUG_BIN=rustc --out-dir $(DEBUG_BIN_DIR) $(DEBUG_RUSTC_FLAGS) $(1)
DEBUG_RLIB=rustc --out-dir $(DEBUG_RLIB_DIR) $(DEBUG_RUSTC_FLAGS) $(1)

RELEASE_BIN=rustc --out-dir $(RELEASE_BIN_DIR) $(RELEASE_RUSTC_FLAGS) $(1)
RELEASE_RLIB=rustc --out-dir $(RELEASE_RLIB_DIR) $(RELEASE_RUSTC_FLAGS) $(1)

TEST_BIN=rustc --out-dir $(TEST_BIN_DIR) $(TEST_RUSTC_FLAGS) $(1)
TEST_BIN=rustc --out-dir $(TEST_BIN_DIR) $(TEST_RUSTC_FLAGS) $(1)

RUN_TEST=$(1) --test
RUN_BENCH=$(1) --bench
