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

SRC=$(wildcard src/*.rs)
PROBDIR=$(wildcard src/prob*_*)
MODSRC=$(patsubst %,%/mod.rs,$(PROBDIR))
DEPSRC=$(wildcard src/common/*.rs)

TARGET=$(patsubst src/%,bin/%,$(SRC:.rs=$(EXEEXT)))
TEST=$(patsubst src/%,bin/%,$(SRC:.rs=.test$(EXEEXT)))

RUSTC_FLAGS=
RUSTC_DEBUG_FLAGS=
RUSTC_RELEASE_FLAGS=--opt-level 3
RUSTC_PARSE_FLAGS=--no-trans

.PHONY: debug release parse test clean

debug: RUSTC_FLAGS+=$(RUSTC_DEBUG_FLAGS)
debug: $(TARGET)

release: RUSTC_FLAGS+=$(RUSTC_RELEASE_FLAGS)
release: $(TARGET)

parse: RUSTC_FLAGS+=$(RUSTC_PARSE_FLAGS)
parse: $(TARGET)

bin/%$(EXEEXT): src/%.rs $(MODSRC) $(DEPSRC)
	rustc $(RUSTC_FLAGS) $< -o $@

bin/%.test$(EXEEXT): src/%.rs $(MODSRC) $(DEPSRC)
	rustc --test $< -o $@

.SECONDEXPANSION:
$(MODSRC): $$(wildcard $$(@D)/prob*.rs)
	./etc/genmod $(@D) > $@

test: $(TEST)
	@for exe in $(TEST); do echo "$$exe"; ./$$exe; done

clean:
	$(RM) $(TARGET) $(TEST) $(MODSRC)
