UNAME=$(shell uname)

ifeq ($(OS), Windows_NT)
LIBEXT=.dll
EXEEXT=.exe
else ifeq ($(UNAME), Linux)
LIBEXT=.so
EXEEXT=
else ifeq ($(UNAME), Darwin)
LIBEXT=.dylib
EXEEXT=
else
$(error Unknown OS $(OS) or UNAME $(UNAME))
endif

BINDIR=
LIBDIR=

VPATH=src

EULER_SRC=src/euler/main.rs
COMMON_SRC=src/common/lib.rs
DATA_SRC=src/data/lib.rs
MATH_SRC=src/math/lib.rs
PROB_SRC=$(sort $(wildcard src/prob*.rs))
MOD_SRC=src/euler/problem.rs
ALL_SRC=$(EULER_SRC) $(COMMON_SRC) $(DATA_SRC) $(MATH_SRC) $(PROB_SRC) $(MOD_SRC)

DEPEND=depend.mk

DEBUG_BINDIR=bin/debug
DEBUG_LIBDIR=lib/debug
RELEASE_BINDIR=bin/release
RELEASE_LIBDIR=lib/release
TEST_BINDIR=bin/test
TEST_LIBDIR=lib/debug

TARGET=$(BINDIR)/euler$(EXEEXT)
LIB_TEST=$(patsubst %,$(BINDIR)/%.test$(EXEEXT),common data math)
ALL_TEST=$(LIBTEST) $(patsubst %,$(BINDIR)/%.test$(EXEEXT),euler $(patsubst %.rs,%,$(notdir $(PROB_SRC))))

RUSTC_FLAGS=-L $(LIBDIR)
RUSTC_DEBUG_FLAGS=
RUSTC_RELEASE_FLAGS=--opt-level 3

.PHONY: debug release test depend clean debug_bin release_bin test_binary

debug:
	make BINDIR=$(DEBUG_BINDIR) LIBDIR=$(DEBUG_LIBDIR) debug_bin
release:
	make BINDIR=$(RELEASE_BINDIR) LIBDIR=$(RELEASE_LIBDIR) release_bin
test:
	make BINDIR=$(TEST_BINDIR) LIBDIR=$(TEST_LIBDIR) test_bin
libtest:
	make BINDIR=$(TEST_BINDIR) LIBDIR=$(TEST_LIBDIR) libtest_bin
depend: $(DEPEND)


clean:
	$(RM) -r $(MOD_SRC) $(DEPEND)
	$(RM) -r $(DEBUG_BINDIR)/* $(DEBUG_LIBDIR)/*
	$(RM) -r $(RELEASE_BINDIR)/* $(RELEASE_LIBDIR)/*
	$(RM) -r $(TEST_BINDIR)/* $(TEST_LIBDIR)/*

$(DEPEND): $(ALL_SRC)
	./etc/gendep > $@
$(MOD_SRC): $(PROB_SRC)
	./etc/genmod ./src > $@



debug_bin: RUSTC_FLAGS+=$(RUSTC_DEBUG_FLAGS)
debug_bin: $(TARGET)
release_bin: RUSTC_FLAGS+=$(RUSTC_RELEASE_FLAGS)
release_bin: $(TARGET)
test_bin: RUSTC_FLAGS+=$(RUSTC_DEBUG_FLAGS)
test_bin: $(ALL_TEST)
	@for exe in $(ALL_TEST); do echo "$$exe"; ./$$exe || exit 1; done
libtest_bin: RUSTC_FLAGS+=$(RUSTC_DEBUG_FLAGS)
libtest_bin: $(LIB_TEST)
	@for exe in $(LIB_TEST); do echo "$$exe"; ./$$exe || exit 1; done

-include $(DEPEND)


define genexe
	rustc -o $1 $(RUSTC_FLAGS) $2
endef
$(BINDIR)/euler$(EXEEXT):
	$(call genexe, $@, $(EULER_SRC))
$(BINDIR)/%$(EXEEXT): %.rs
	$(call genext, $@, $(patsubst $(BINDIR)/%$(EXEEXT),src/%.rs,$@))


define genlib
	$(RM) $(patsubst %$(LIBEXT),%-*$(LIBEXT),$1)
	rustc --lib --out-dir $(LIBDIR) $(RUSTC_FLAGS) $2
	touch $1
endef

$(LIBDIR)/libcommon$(LIBEXT):
	$(call genlib, $@, $(COMMON_SRC))
$(LIBDIR)/libdata$(LIBEXT):
	$(call genlib, $@, $(DATA_SRC))
$(LIBDIR)/libmath$(LIBEXT):
	$(call genlib, $@, $(MATH_SRC))
$(LIBDIR)/lib%$(LIBEXT):
	$(call genlib, $@, $(patsubst $(LIBDIR)/lib%$(LIBEXT),src/%.rs,$@))


define gentest
	rustc --test -o $1 $(RUSTC_FLAGS) $2
endef
$(BINDIR)/euler.test$(EXEEXT):
	$(call gentest, $@, $(EULER_SRC))
$(BINDIR)/common.test$(EXEEXT):
	$(call gentest, $@, $(COMMON_SRC))
$(BINDIR)/data.test$(EXEEXT):
	$(call gentest, $@, $(DATA_SRC))
$(BINDIR)/math.test$(EXEEXT):
	$(call gentest, $@, $(MATH_SRC))
$(BINDIR)/%.test$(EXEEXT):
	$(call gentest, $@, $(patsubst $(BINDIR)/%.test$(EXEEXT),src/%.rs,$@))

