# Echo Bot - Nodejs over jsonrpc api over stdio

jsonrpc is the new way to speak with our core library, it is faster (both to use and to develop) and returns better errors than the cffi.
This example uses the new reproducible deltachat-rpc-server binary and comunicates with it over stdio pipes.

## Run the bot:

you need nodejs version `>=18`.
install dependencies with `npm install`.

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

on linux and mac you can also do it in one line:

```
ADDR=$yourEmail MAIL_PW=$yourPassword node .
```

> you only need to provide the email credentials the first time you start it, they won't have an effect after the account is already configured.

#### NEW! get a quick chatmail account

Chatmail is a server configuration optimized for DeltaChat.

- advantages:
    - fast testing / super quick sign up without personal data
    - very fast
- disadvantage:
    - forces you to be on the same chatmail instance, unless you verify the bots contact with the the vergification code.


To set-up an account via chatmail:
```
CHATMAIL_QR=dcaccount:https://nine.testrun.org/new node .
```

### Useful Links

- Generated client code for the jsonrpc: https://github.com/deltachat/deltachat-core-rust/blob/master/deltachat-jsonrpc/typescript/generated/client.ts
- Generated typescript types for the jsonrpc: https://github.com/deltachat/deltachat-core-rust/blob/master/deltachat-jsonrpc/typescript/generated/types.ts
- Sourcecode of the jsonrpc client: https://github.com/deltachat/deltachat-core-rust/blob/master/deltachat-jsonrpc/typescript/
- Sourcecode of the deltachat-rpc-server to javascript bindings: https://github.com/deltachat/deltachat-core-rust/blob/master/deltachat-rpc-server/npm-package


