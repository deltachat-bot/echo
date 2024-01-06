# Go deltabot-cli

This example is about creating a bot with the bot framework [deltabot-cli-go](https://github.com/deltachat-bot/deltabot-cli-go/), which gives a hook based interface and a cli interface to configure your bot.

> This is a go framework for bots, see [../README.md](../README.md) for other approaches to write bot in go.

## Usage

> Per default the bot data and configuration is stored in your user directory
> (`Library/Application Support/bot_name/`, `.config/bot_name/`, on windows probably in `%APPDATA%` <!-- todo the location on windows needs to be checked -->).
>
> You can change configuration location with `--folder` flag (`--folder PATH` or `-f PATH`).

### Setup

### Installing deltachat-rpc-server

For the bot to work, first `deltachat-rpc-server` program needs to
be installed and available in your `PATH`. To install it from source run:

```sh
cargo install --git https://github.com/deltachat/deltachat-core-rust/ deltachat-rpc-server
```

For more info and pre-built binaries check:
https://github.com/deltachat/deltachat-core-rust/tree/master/deltachat-rpc-server

#### Install

```sh
go mod tidy
```

#### Configure

```sh
go run ./echobot.go init address@example.com password
```

### Start

```sh
go run ./echobot.go serve
```
