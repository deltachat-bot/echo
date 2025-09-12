# Python

## Installation

```sh
# Optional create virtualenv
pip3 install virtualenv
virtualenv .venv
source .venv/bin/activate

# install rpc server
pip install deltachat-rpc-server
# install client
pip install deltachat-rpc-client
```

For alternative installation instructions refer to https://py.delta.chat/jsonrpc/install.html#install (building from souce or download prbuilds manually).

## Usage

set credentials
```sh
# mac & linux
export MAIL_ADDR=email@example.com
export MAIL_PW="my_password"
# windows
set MAIL_ADDR=email@example.com
set MAIL_PW="my_password"
```

start the bot:
```
python echo_bot.py /tmp/dc-accounts
```

or start the version made with the hook system:
```
python ./echo_bot_with_hooks.py --email $MAIL_ADDR --password $MAIL_PW
```

### References

- Python bindings documentation https://py.delta.chat
- https://py.delta.chat/jsonrpc/examples.html
- source code of python bindings https://github.com/deltachat/deltachat-core-rust/tree/master/deltachat-rpc-client
