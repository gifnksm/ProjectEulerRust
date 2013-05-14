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

PROBDIR=$(wildcard src/prob*_*)
VPATH=$(PROBDIR)

TARGETSRC=src/euler.rs
COMMONSRC=src/common/lib.rs
PROBSRC=$(sort $(wildcard src/prob*_*/prob*.rs))
MODSRC=src/problem.rs

BINDIR=bin
LIBDIR=lib

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

.PHONY: debug release test clean
.PRECIOUS: $(PROBLIB) $(COMMONLIB)

debug: RUSTC_FLAGS+=$(RUSTC_DEBUG_FLAGS)
debug: $(TARGET)

release: RUSTC_FLAGS+=$(RUSTC_RELEASE_FLAGS)
release: $(TARGET)

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

test: $(TEST)
	@for exe in $(TEST); do echo "$$exe"; ./$$exe || exit 1; done

clean:
	$(RM) $(MODSRC) $(BINDIR)/* $(LIBDIR)/*
