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

SRC=$(wildcard *.rs)
DEPSRC=$(wildcard prob*_*/*.rs common/*.rs)

TARGET=$(SRC:.rs=$(EXEEXT))
TEST=$(SRC:.rs=.test$(EXEEXT))

RUSTC_FLAGS=
RUSTC_DEBUG_FLAGS=-Z debug-info
RUSTC_RELEASE_FLAGS=--opt-level 3

.PHONY: debug release test clean

debug: RUSTC_FLAGS+=$(RUSTC_DEBUG_FLAGS)
debug: $(TARGET)

release: RUSTC_FLAGS+=$(RUSTC_RELEASE_FLAGS)
release: $(TARGET)

%$(EXEEXT): %.rs $(DEPSRC)
	rustc $(RUSTC_FLAGS) $< -o $@

%.test$(EXEEXT): %.rs $(DEPSRC)
	rustc --test $< -o $@

test: $(TEST)
	@for exe in $(TEST); do echo "$$exe"; ./$$exe; done

clean:
	$(RM) $(TARGET) $(TEST)
