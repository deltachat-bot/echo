# Echo Bot - Node.js

Node.js echo bot uses JSON-RPC API over stdio.

JSON-RPC API is the new way to speak with our core library.
It is faster (both to use and to develop) and returns better errors than the C API.
This example uses reproducible deltachat-rpc-server binary and comunicates with it over stdio pipes.

## Run the bot

You need [Node.js](https://nodejs.org/) version `>=18`.
Install dependencies with `npm install`.

Set your credentials as enviroment variables:

```sh
# on Mac and Linux
export ADDR=$yourEmail
export MAIL_PW=$yourPassword
# on windows
## TODO
```

then start with:

```
node .
```

on Linux and macOS you can also do it in one line:

```
ADDR=$yourEmail MAIL_PW=$yourPassword node .
```

> you only need to provide the email credentials the first time you start it, they won't have an effect after the account is already configured.

#### Get a chatmail account

[Chatmail](https://github.com/deltachat/chatmail) is a server configuration optimized for Delta Chat.

- advantages:
    - fast testing / super quick sign up without personal data
    - very fast
- disadvantage:
    - forces you to be on the same chatmail instance, unless you verify the bots contact with the the verification code.

To set-up an account via chatmail:
```
CHATMAIL_QR=dcaccount:https://nine.testrun.org/new node .
```

### Useful Links

- [JSON-RPC API documentation](https://jsonrpc.delta.chat/)
- Source code of the JSON-RPC client: https://github.com/deltachat/deltachat-core-rust/blob/main/deltachat-jsonrpc/typescript/
- Source code of the deltachat-rpc-server to JavaScript bindings: https://github.com/deltachat/deltachat-core-rust/blob/main/deltachat-rpc-server/npm-package
