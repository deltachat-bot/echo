# Echo Bot - Nodejs over jsonrpc api over napi-rs

jsonrpc is the new way to speak with our core library, it is faster (both to use and to develop) and returns better errors than the cffi.

## Run the bot:

you need nodejs version `>=16` and if you use typescript you need version `>=4.7`.
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

### Useful Links

- Generated client code for the jsonrpc: https://github.com/deltachat/deltachat-core-rust/blob/master/deltachat-jsonrpc/typescript/generated/client.ts
- Generated typescript types for the jsonrpc: https://github.com/deltachat/deltachat-core-rust/blob/master/deltachat-jsonrpc/typescript/generated/types.ts
- Sourcecode of the jsonrpc client: https://github.com/deltachat/deltachat-core-rust/blob/master/deltachat-jsonrpc/typescript/
- Sourcecode of the core to nodejs bindings: https://github.com/deltachat/napi-jsonrpc
