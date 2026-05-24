# Build the cdylib and run the C test suite (tests/c/lib_test.c).

CC ?= cc
TARGET_DIR := target/debug
TEST_SRC := tests/c/lib_test.c
TEST_BIN := $(TARGET_DIR)/lib_test

# macOS resolves shared libraries via DYLD_LIBRARY_PATH, Linux via LD_LIBRARY_PATH.
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	LIB_PATH_VAR := DYLD_LIBRARY_PATH
else
	LIB_PATH_VAR := LD_LIBRARY_PATH
endif

.PHONY: ctest
ctest:
	cargo build
	$(CC) $(TEST_SRC) -o $(TEST_BIN) -llymui_c -L$(TARGET_DIR)
	$(LIB_PATH_VAR)=$(TARGET_DIR) $(TEST_BIN)

.PHONY: clean
clean:
	rm -f $(TEST_BIN)
