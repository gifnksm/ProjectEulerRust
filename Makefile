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
LD_FLAGS=
TEST_RUSTC_FLAGS=

all: $(TARGET)

%$(EXEEXT): %.rs $(DEPSRC)
	rustc $(RUSTC_FLAGS) $(LD_FLAGS) $< -o $@

%.test$(EXEEXT): %.rs $(DEPSRC)
	rustc --test $(RUSTC_FLAGS) $(TEST_RUSTC_FLAGS) $(LD_FLAGS) $< -o $@

.PHONY: test clean

test: $(TEST)
	@for exe in $(TEST); do echo "$$exe"; ./$$exe; done

clean:
	$(RM) $(TARGET) $(TEST)
