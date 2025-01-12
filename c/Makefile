CC = gcc
DELTACHAT_PKG_CONFIG = /usr/local/lib64/pkgconfig
DELTACHAT_INCLUDE = /usr/local/include
DELTACHAT_LIB = /usr/local/lib64

.PHONY = all manual pkgconfig 

all:
	@echo if you know the directory of deltachat.pc, do:
	@echo make DELTACHAT_PKG_CONFIG="<directory of deltachat.pc>" pkgconfig
	@echo or if you know the directories of deltachat.h and libdeltachat.*, do:
	@echo make DELTACHAT_INCLUDE="<directory of deltachat.h>" DELTACHAT_LIB="<directory of libdeltachat.*>" manual
	@echo
	@echo if they are in /usr/local/{include,lib64}, you can do either:
	@echo make manual
	@echo or
	@echo make pkgconfig

pkgconfig:
	$(CC) -g -O2 -Wextra -Wconversion -Werror \
  $(shell PKG_CONFIG_PATH="$(DELTACHAT_PKG_CONFIG)" \
    pkg-config --cflags --libs deltachat) \
  -Wl,-rpath,$(shell PKG_CONFIG_PATH=$(DELTACHAT_PKG_CONFIG) pkg-config \
               --libs-only-L deltachat) \
  -lpthread main.c -o deltachat_c_echo_bot

manual:
	$(CC) -g -O2 -Wextra -Wconversion -Werror \
  -I"$(DELTACHAT_INCLUDE)" -L"$(DELTACHAT_LIB)" -ldeltachat \
  -Wl,-rpath,"$(DELTACHAT_LIB)" \
  -lpthread main.c -o deltachat_c_echo_bot

