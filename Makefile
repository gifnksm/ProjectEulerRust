SRC_DIR    = mk_src
PROB_SRC   = $(sort $(wildcard $(SRC_DIR)/prob*.rs))
MOD_SRC    = $(SRC_DIR)/euler/problem_list.rs
ALL_SRC    = $(wildcard $(SRC_DIR)/*.rs) $(wildcard $(SRC_DIR)/*/*.rs)

DEPEND_DIR=depend
DEBUG_RLIB_DIR=lib/debug
DEBUG_BIN_DIR=bin/debug

RELEASE_RLIB_DIR=lib/release
RELEASE_BIN_DIR=bin/release

DOC_DIR=doc
DEPEND=$(DEPEND_DIR)/all.mk

RUSTC_FLAGS = \
	-W bad-style \
	-W unused \
	-W unused-qualifications \
	-W unused-typecasts
DEBUG_RUSTC_FLAGS   = $(RUSTC_FLAGS) -L $(DEBUG_RLIB_DIR) #-g
RELEASE_RUSTC_FLAGS = $(RUSTC_FLAGS) -L $(RELEASE_RLIB_DIR) --opt-level 3

DEBUG_rustc-serialize = $(DEBUG_RLIB_DIR)/librustc-serialize.rlib
RELEASE_rustc-serialize = $(RELEASE_RLIB_DIR)/librustc-serialize.rlib

DEBUG_num = $(DEBUG_RLIB_DIR)/libnum.rlib
RELEASE_num = $(RELEASE_RLIB_DIR)/libnum.rlib

.PHONY: debug release test bench doc depend mostlyclean clean

debug:
release:
test:
bench:
depend: $(DEPEND)
doc:

mostlyclean:
	$(RM) $(DEBUG_BIN_DIR)/* $(RELEASE_BIN_DIR)/*
	$(RM) $(DEBUG_RLIB_DIR)/*.rlib $(RELEASE_RLIB_DIR)/*.rlib

clean: mostlyclean
	$(RM) -r $(MOD_SRC) $(DEPEND_DIR)/*
	$(RM) -r $(DOC_DIR)

$(DEPEND): $(ALL_SRC) $(MOD_SRC)
	./etc/mkdepend $(SRC_DIR) > $@
$(MOD_SRC): $(PROB_SRC)
	./etc/mkproblist $(SRC_DIR) > $@

ifneq "$(MAKECMDGOALS)" "clean"
ifneq "$(MAKECMDGOALS)" "mostlyclean"
-include $(DEPEND)
endif
endif

DEBUG_BIN=rustc $(DEBUG_RUSTC_FLAGS) $(1) -o $@
DEBUG_RLIB=rustc $(DEBUG_RUSTC_FLAGS) $(1) --out-dir $(DEBUG_RLIB_DIR)
DEBUG_TEST=rustc --test $(DEBUG_RUSTC_FLAGS) $(1) -o $@

RELEASE_BIN=rustc $(RELEASE_RUSTC_FLAGS) $(1) -o $@
RELEASE_RLIB=rustc $(RELEASE_RUSTC_FLAGS) $(1) --out-dir $(RELEASE_RLIB_DIR)
RELEASE_TEST=rustc --test $(RELEASE_RUSTC_FLAGS) $(1) -o $@

DOC=rustdoc -L $(DEBUG_RLIB_DIR) $(1) -o $(DOC_DIR)

RUN_TEST=$(1) --test
RUN_BENCH=$(1) --bench

$(DEBUG_rustc-serialize):
	cargo build -p rustc-serialize
	cp target/deps/librustc-serialize-*.rlib $@

$(RELEASE_rustc-serialize):
	cargo build -p rustc-serialize --release
	cp target/release/deps/librustc-serialize-*.rlib $@


$(DEBUG_num): $(DEBUG_rustc-serialize)
$(RELEASE_num): $(RELEASE_rustc-serialize)

$(DEBUG_num):
	cargo build -p num
	cp target/deps/libnum-*.rlib $@

$(RELEASE_num):
	cargo build -p num --release
	cp target/release/deps/libnum-*.rlib $@
