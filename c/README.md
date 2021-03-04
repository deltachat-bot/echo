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

> Info: There is no uninstall yet, use `sudo rm` to remove the files cmake created.

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
