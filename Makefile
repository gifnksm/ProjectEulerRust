PROB_SRC   = $(sort $(wildcard src/prob*.rs))
MOD_SRC    = src/euler/problem_list.rs
ALL_SRC    = $(wildcard src/*.rs) $(wildcard src/*/*.rs)

DEPEND_DIR=depend
DEBUG_RLIB_DIR=lib/debug
DEBUG_BIN_DIR=bin/debug

RELEASE_RLIB_DIR=lib/release
RELEASE_BIN_DIR=bin/release

DEPEND=$(DEPEND_DIR)/all.mk

RUSTC_FLAGS = -D warnings
DEBUG_RUSTC_FLAGS   = $(RUSTC_FLAGS) -L $(DEBUG_RLIB_DIR)
RELEASE_RUSTC_FLAGS = $(RUSTC_FLAGS) -L $(RELEASE_RLIB_DIR) --opt-level 3

.PHONY: debug release test bench depend clean

debug:
release:
test:
bench:
depend: $(DEPEND)

clean:
	$(RM) -r $(MOD_SRC) $(DEPEND_DIR)/*
	$(RM) $(DEBUG_BIN_DIR)/* $(RELEASE_BIN_DIR)/*
	$(RM) $(DEBUG_RLIB_DIR)/*.rlib $(RELEASE_RLIB_DIR)/*.rlib

$(DEPEND): $(ALL_SRC) $(MOD_SRC)
	./etc/mkdepend > $@
$(MOD_SRC): $(PROB_SRC)
	./etc/mkproblist ./src > $@

ifneq "$(MAKECMDGOALS)" "clean"
-include $(DEPEND)
endif

DEBUG_BIN=rustc $(DEBUG_RUSTC_FLAGS) $(1) -o $@
DEBUG_RLIB=rustc $(DEBUG_RUSTC_FLAGS) $(1) --out-dir $(DEBUG_RLIB_DIR)
DEBUG_TEST=rustc --test $(DEBUG_RUSTC_FLAGS) $(1) -o $@

RELEASE_BIN=rustc $(RELEASE_RUSTC_FLAGS) $(1) -o $@
RELEASE_RLIB=rustc $(RELEASE_RUSTC_FLAGS) $(1) --out-dir $(RELEASE_RLIB_DIR)
RELEASE_TEST=rustc --test $(RELEASE_RUSTC_FLAGS) $(1) -o $@

RUN_TEST=$(1) --test
RUN_BENCH=$(1) --bench
