# Python

> There are 2 alternative options availible:
>
> - [using deltabot](../python_deltabot_plugin) (bot framework, includes features as chat command parsing)
> - [using simplebot](../python_simplebot_plugin) (simplebot is a maintained fork of deltabot with many plugins availible)

## Instalation

```sh
# Optional create virtualenv
pip install virtualenv
virtualenv .venv
source .venv/bin/activate
pip3 install -U pip wheel

# install deltachat
pip3 install --pre -U -i https://m.devpi.net/dc/master deltachat # see https://github.com/deltachat/deltachat-core-rust/tree/master/python if it doesn't work for instructions on how to install it from source)
```

## Usage

```sh
python echo_bot.py /tmp/db --email ADDRESS --password PASSWORD
```

### References

- all availible hooks can be found in https://github.com/deltachat/deltachat-core-rust/blob/master/python/src/deltachat/hookspec.py
- Python bindings documentation py.delta.chat
- https://py.delta.chat/examples.html
- ourcecode of python bindings https://github.com/deltachat/deltachat-core-rust/tree/master/python
