# Echo Bot - Go

### Installing deltachat-rpc-server

For the bot to work, first `deltachat-rpc-server` program needs to
be installed and available in your `PATH`. To install it from source run:

```sh
cargo install --git https://github.com/deltachat/deltachat-core-rust/ deltachat-rpc-server
```

For more info and pre-built binaries check:
https://github.com/deltachat/deltachat-core-rust/tree/master/deltachat-rpc-server

### Using the bot

To run the bot, the first time you need to pass the credentials
as command line arguments:

```sh
go run . $yourEmail $yourPassword
```

This will create a subdirectory called `accounts` in the current
working directory, this is where deltachat stores the state.  The
credentials will be stored there so further invocations do not need
them specified again:

```sh
go run .
```

Open a chat with the bot address in your Delta Chat and write some messages
to test the bot.
