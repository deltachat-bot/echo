> **Info**: [`deltachat-node-bot-base`](https://github.com/deltachat-bot/deltachat-node-bot-base) is **currently unmaintained**, so if you want all newer features you should go with [`@deltachat/napi-jsonrpc`](https://github.com/deltachat/napi-jsonrpc). see [@deltachat/napi-jsonrpc Example](../nodejs_napi_jsonrpc)

# Echo Bot - Nodejs Bot Base

> This is an makes use of [`deltachat-node-bot-base`](https://github.com/deltachat-bot/deltachat-node-bot-base) an abstraction Layer over [`deltachat-node`](https://github.com/deltachat/deltachat-node) which does the initial setup for you. It is opinionated (makes some design decisions for you, like the used config module) -> If you want more control look at our other echo-bot example that uses `deltachat-node` directly: [Advanced Example](../nodejs_cffi)

## First Time Setup

install dependencies with `npm install`

Configure the bot by writing its email-address and password into `config/local.json` like this:

```json
{
  "email_address": "bot@example.net",
  "email_password": "secretandsecure"
}
```

## Start The Bot

run the bot:

```
node .
```

## Questions

If you got any questions reach out to us: https://bots.delta.chat/contact.html
