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

all: $(TARGET)

%$(EXEEXT): %.rs $(DEPSRC)
	rustc --opt-level 3 $< -o $@

%.test$(EXEEXT): %.rs $(DEPSRC)
	rustc --test $< -o $@

.PHONY: test clean

test: $(TEST)
	@for exe in $(TEST); do echo "$$exe"; ./$$exe; done

clean:
	$(RM) $(TARGET) $(TEST)
