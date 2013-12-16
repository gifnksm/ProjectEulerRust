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

EULER_SRC=src/euler/main.rs
COMMON_SRC=src/common/lib.rs
DATA_SRC=src/data/lib.rs
MATH_SRC=src/math/lib.rs
PROB_SRC=$(sort $(wildcard src/prob*.rs))
MOD_SRC=src/euler/problem.rs
ALL_SRC=$(wildcard src/*.rs) $(wildcard src/*/*.rs)

DEPEND=depend.mk

define debugexe
bin/debug/$1$(EXEEXT)
endef
define releaseexe
bin/release/$1$(EXEEXT)
endef
define testexe
bin/test/$1.test$(EXEEXT)
endef

define debuglib
lib/debug/lib$1$(LIBEXT)
endef
define releaselib
lib/release/lib$1$(LIBEXT)
endef

DEBUG_TARGET=$(call debugexe,euler)
RELEASE_TARGET=$(call releaseexe,euler)
LIBTEST_TARGET=$(patsubst %,$(call testexe,%),common data math)
ALLTEST_TARGET=$(LIBTEST) $(patsubst %,$(call testexe,%),euler $(patsubst %.rs,%,$(notdir $(PROB_SRC))))

RUSTC_DEBUG_FLAGS=-L lib/debug
RUSTC_RELEASE_FLAGS=--opt-level 3 -L lib/release
RUSTC_TEST_FLAGS=-L lib/debug

.PHONY: debug release test libtest depend clean

debug: $(DEBUG_TARGET)
release: $(RELEASE_TARGET)
test: $(ALLTEST_TARGET)
	@for exe in $^; do ./$$exe || exit 1; done
libtest: $(LIBTEST_TARGET)
	@for exe in $^; do ./$$exe || exit 1; done
depend: $(DEPEND)

clean:
	$(RM) -r $(MOD_SRC) $(DEPEND)
	$(RM) -r $(call debugexe,*) $(call releaseexe,*) $(call testexe,*)
	$(RM) -r $(call debuglib,*) $(call releaselib,*)

$(DEPEND): $(ALL_SRC) $(MOD_SRC)
	./etc/gendep > $@
$(MOD_SRC): $(PROB_SRC)
	./etc/genmod ./src > $@

-include $(DEPEND)

define gen_debugexe
	rustc -o $1 $(RUSTC_DEBUG_FLAGS) $2
endef
define gen_releaseexe
	rustc -o $1 $(RUSTC_RELEASE_FLAGS) $2
endef
define gen_debuglib
	$(RM) $(patsubst %$(LIBEXT),%-*$(LIBEXT),$1)
	rustc --lib --out-dir lib/debug $(RUSTC_DEBUG_FLAGS) $2
	touch $1
endef
define gen_releaselib
	$(RM) $(patsubst %$(LIBEXT),%-*$(LIBEXT),$1)
	rustc --lib --out-dir lib/release $(RUSTC_RELEASE_FLAGS) $2
	touch $1
endef
define gen_testexe
	rustc --test -o $1 $(RUSTC_TEST_FLAGS) $2
endef

$(call debugexe,euler):
	$(call gen_debugexe,$@,$(EULER_SRC))
$(call releaseexe,euler):
	$(call gen_releaseexe,$@,$(EULER_SRC))

$(call debuglib,common):
	$(call gen_debuglib,$@,$(COMMON_SRC))
$(call debuglib,data):
	$(call gen_debuglib,$@,$(DATA_SRC))
$(call debuglib,math):
	$(call gen_debuglib,$@,$(MATH_SRC))
$(call debuglib,%):
	$(call gen_debuglib,$@,$(patsubst $(call debuglib,%),src/%.rs,$@))
$(call releaselib,common):
	$(call gen_releaselib,$@,$(COMMON_SRC))
$(call releaselib,data):
	$(call gen_releaselib,$@,$(DATA_SRC))
$(call releaselib,math):
	$(call gen_releaselib,$@,$(MATH_SRC))
$(call releaselib,%):
	$(call gen_releaselib,$@,$(patsubst $(call releaselib,%),src/%.rs,$@))

$(call testexe,euler):
	$(call gen_testexe,$@,$(EULER_SRC))
$(call testexe,common):
	$(call gen_testexe,$@,$(COMMON_SRC))
$(call testexe,data):
	$(call gen_testexe,$@,$(DATA_SRC))
$(call testexe,math):
	$(call gen_testexe,$@,$(MATH_SRC))
$(call testexe,%):
	$(call gen_testexe,$@,$(patsubst $(call testexe,%),src/%.rs,$@))

