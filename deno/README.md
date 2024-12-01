# Echo Bot - Deno

[Deno](https://deno.com/) is an alternative to Node.js. Deno has
[TypeScript](https://www.typescriptlang.org) support out of the box, good
default tooling, and a strict focus on security through it's permission system.

For Deno the native prebuilds in the npm package do not work (they do not get
installed / found), so you need to install `deltachat-rpc-server` yourself to
`$PATH` with:

```sh
cargo install --git https://github.com/deltachat/deltachat-core-rust deltachat-rpc-server
```

Or you download it from the releases page on
https://github.com/deltachat/deltachat-core-rust/releases/ and point to it with
the `DELTA_CHAT_RPC_SERVER` environment variable:

```sh
# make it executable
chmod +x ./deltachat-rpc-server-aarch64-macos
# set the `DELTA_CHAT_RPC_SERVER` environment variable
export DELTA_CHAT_RPC_SERVER=./deltachat-rpc-server-aarch64-macos
```

run `index.js`:

```sh
CHATMAIL_QR=dcaccount:https://nine.testrun.org/new deno run --allow-env --allow-read --allow-run=deltachat-rpc-server index.js
```
