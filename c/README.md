# Echo Bot - C

## Requirements

- This readme describes the process for compiling on **linux**, if you are on another platform your steps might vary. If you figgured out how to do it on your platform feel free to sumbit a pr to add your method to this readme.
- You need `libdeltachat`,`git`, `cmake`, `gcc` and `pkg-config` also `rustup` if you compile `libdeltachat` on your own

### Installing libdeltachat from source

```sh
git clone https://github.com/deltachat/deltachat-core-rust
cd deltachat-core-rust
cmake -B build . && cmake --build build && sudo cmake --install build
```

> Info: To uninstall, run `sudo xargs -d\\n rm -f <build/install_manifest.txt` in the same directory

## Usage

### Compile

```sh
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
gcc main.c -o main $(pkg-config --cflags --libs deltachat) -lpthread
```

### Run

```sh
LD_LIBRARY_PATH=/usr/local/lib addr=$yourEmail mailpw=$yourPassword ./main
```

### Format code

```sh
clang-format -i main.c
```

You might need to install `clang-format` first.

### Quirks

> There is no way to terminate it currently. I wanted to propose a SIGTERM handler, but realized doing it properly is more difficult than it should be: deltachat/deltachat-core-rust#2280
> So for the purpose of example it's easier to assume the bot will be running forever.
>
> ~ link2xt on https://github.com/deltachat-bot/echo/pull/13#issuecomment-790594993

### Useful Links

- Documentation https://c.delta.chat/
