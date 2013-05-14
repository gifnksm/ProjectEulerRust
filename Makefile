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

PROBDIR=$(wildcard src/prob*_*)
VPATH=$(PROBDIR)

TARGETSRC=src/euler.rs
COMMONSRC=src/common/lib.rs
PROBSRC=$(sort $(wildcard src/prob*_*/prob*.rs))
MODSRC=src/problem.rs

DEBUG_BINDIR=bin/debug
DEBUG_LIBDIR=lib/debug
RELEASE_BINDIR=bin/release
RELEASE_LIBDIR=lib/release
TEST_BINDIR=bin/test
TEST_LIBDIR=lib/debug

TARGET=$(BINDIR)/euler$(EXEEXT)
COMMONLIB=$(LIBDIR)/libcommon$(LIBEXT)
PROBLIB=$(patsubst prob%.rs,$(LIBDIR)/libprob%$(LIBEXT),$(notdir $(PROBSRC)))

TARGETTEST=$(BINDIR)/euler.test$(EXEEXT)
COMMONTEST=$(BINDIR)/common.test$(EXTEXT)
PROBTEST=$(patsubst prob%.rs,$(BINDIR)/prob%.test$(EXEEXT),$(notdir $(PROBSRC)))
TEST=$(TARGETTEST) $(COMMONTEST) $(PROBTEST)

TARGETDEP=$(TARGETSRC) $(MODSRC) $(COMMONLIB) $(PROBLIB)
COMMONDEP=$(wildcard src/common/*.rs)
PROBDEP=$(COMMONLIB)

RUSTC_FLAGS=-L $(LIBDIR)
RUSTC_DEBUG_FLAGS=
RUSTC_RELEASE_FLAGS=--opt-level 3

.PHONY: debug release test clean debug_bin release_bin test_binary
.PRECIOUS: $(PROBLIB) $(COMMONLIB)

debug:
	make BINDIR=$(DEBUG_BINDIR) LIBDIR=$(DEBUG_LIBDIR) debug_bin

release:
	make BINDIR=$(RELEASE_BINDIR) LIBDIR=$(RELEASE_LIBDIR) release_bin

test: RUSTC_FLAGS+=$(RUSTC_DEBUG_FLAGS)
test: BINDIR=$(TEST_BINDIR)
test: LIBDIR=$(TEST_LIBDIR)
test:
	make BINDIR=$(TEST_BINDIR) LIBDIR=$(TEST_LIBDIR) test_bin
	@for exe in $(TEST); do echo "$$exe"; ./$$exe || exit 1; done

debug_bin: RUSTC_FLAGS+=$(RUSTC_DEBUG_FLAGS)
debug_bin: $(TARGET)
release_bin: RUSTC_FLAGS+=$(RUSTC_RELEASE_FLAGS)
release_bin: $(TARGET)
test_bin: $(TEST)

$(TARGET): $(TARGETDEP)
	rustc -o $@ $(RUSTC_FLAGS) $(TARGETSRC)
$(TARGETTEST): $(TARGETDEP)
	rustc --test -o $@ $(RUSTC_FLAGS) $(TARGETSRC)

$(COMMONLIB): $(COMMONDEP)
	rustc --lib --out-dir $(LIBDIR) $(RUSTC_FLAGS) $(COMMONSRC)
	touch $@
$(COMMONTEST): $(COMMONDEP)
	rustc --test -o $@ $(RUSTC_FLAGS) $(COMMONSRC)

$(LIBDIR)/libprob%.so: prob%.rs $(PROBDEP)
	rustc --lib --out-dir $(LIBDIR) $(RUSTC_FLAGS) $<
	touch $@
$(BINDIR)/prob%.test$(EXEEXT): prob%.rs $(PROBDEP)
	rustc --test -o $@ $(RUSTC_FLAGS) $<

$(MODSRC): $(PROBSRC)
	./etc/genmod $(PROBDIR) > $@

clean:
	$(RM) $(MODSRC) $(DEBUG_BINDIR)/* $(DEBUG_LIBDIR)/* $(RELEASE_BINDIR)/* $(RELEASE_LIBDIR)/* $(TEST_BINDIR)/* $(TEST_LIBDIR)/*
