# Python

> There are 2 alternative options available:
>
> - [using deltabot](../python_deltabot_plugin) (bot framework, includes features as chat command parsing)
> - [using simplebot](../python_simplebot_plugin) (simplebot is a maintained fork of deltabot with many plugins availible)

## Installation

```sh
# Optional create virtualenv
pip3 install virtualenv
virtualenv .venv
source .venv/bin/activate
pip3 install -U pip wheel

# install deltachat
pip3 install deltachat # if it doesn't work, see https://github.com/deltachat/deltachat-core-rust/tree/master/python for instructions on how to install it from source
```

## Usage

```sh
python echo_bot.py /tmp/db --email ADDRESS --password PASSWORD
```

### References

- all available hooks can be found in https://github.com/deltachat/deltachat-core-rust/blob/master/python/src/deltachat/hookspec.py
- Python bindings documentation py.delta.chat
- https://py.delta.chat/examples.html
- source code of python bindings https://github.com/deltachat/deltachat-core-rust/tree/master/python
