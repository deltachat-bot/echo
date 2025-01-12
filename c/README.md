# C Echo Bot

- This describes a process for compiling a C echo bot, on a *POSIX*-like system, assuming you have installed `libdeltachat.{a.so}` from [DeltaChat Core Rust](https://github.com/deltachat/deltachat-core-rust).
  - It will typically install the `deltachat.h` and `libdeltachat.{a,so}` into `/usr/local/{include,lib,lib64}`.
- If you are on another platform, your steps might vary.
  - Please feel free to submit a PR for platform-specific instructions.

## API Documentation

<https://c.delta.chat/>

## Prerequisites

- `git`
- a C compiler that allows for passing the linker flag `-Wl,-rpath`, e.g., `gcc`
- GNU `make`
- optionally, `pkg-config`
- a Rust tool suite for *DeltaChat Core Rust*
- an installed `libdeltachat.{a.so}` from the *Core*, with:

*either*

- the directory containing `deltachat.pc`
  - usually in `/usr/local/lib/pkgconfig` or `/usr/local/lib64/pkgconfig`

*or*

- the directory containing `deltachat.h`
  - usually in `/usr/local/include`
- the directory of `libdeltachat.*`
  - usually in `/usr/local/lib` or `/usr/local/lib64`

## Installing *DeltaChat Core Rust* from source


### Assuming root access (requires Rust), defaulting to `/usr/local`:

```sh
git clone https://github.com/deltachat/deltachat-core-rust
cd deltachat-core-rust
cmake -B build
cmake --build build
sudo cmake --install build
```

#### To uninstall

```sh
cd deltachat-core-rust
sudo xargs -d\\n rm -i < build/install_manifest.txt
```

### No root access or to change the install location

Use `-DCMAKE_INSTALL_PREFIX="<install path>"` with `cmake -B build`:

```sh
git clone https://github.com/deltachat/deltachat-core-rust
cd deltachat-core-rust
cmake -B build -DCMAKE_INSTALL_PREFIX="<install path>"
cmake --build build
cmake --install build
# or `sudo cmake --install build` if it is a write-protected path
```

#### To uninstall

```sh
cd deltachat-core-rust
rm -i < build/install_manifest.txt
# or `sudo xargs -d\\n rm -i < build/install_manifest.txt` if it is a write-protected path
```

## Compiling the echo bot

### If you know the path to `deltachat.pc` and have `pkg-config`

```sh
git clone https://github.com/deltachat/deltachat-bot/echo
cd echo/c
make PKG_CONFIG_PATH="<path to deltachat.pc>" pkgconfig
```

### Otherwise

```sh
git clone https://github.com/deltachat/deltachat-bot/echo
cd echo/c
make DELTACHAT_INCLUDE="<path to deltachat.h>" \
DELTACHAT_LIB="<path to libdeltachat.*>" manual
```

### If they are in `/usr/local/include` and `/usr/local/lib64`

```sh
make manual
# or `make pkgconfig`
```

## Run

### First time, if you have an email and password for the bot

```sh
cd echo/c
DELTACHAT_C_ECHO_EMAIL="<email>" \
DELTACHAT_C_ECHO_PASSWORD="<password>" \
./deltachat_c_echo_bot
```

This will place the `bot.db` and blob directory in the same 
directory as `deltachat_c_echo_bot`.

### In subsequent runs, assuming `bot.db` is in the same directory as `deltachat_c_echo_bot`

```sh
cd echo/c
./deltachat_c_echo_bot
```

### If you have an existing state database, i.e., `dc.db` or one from a backup

```sh
cd echo/c
mv "<name of existing db, like dc.db>" bot.db
mv "<name of blob directory, like dc.db-blobs>" ./bot.db-blobs
./deltachat_c_echo_bot
```


