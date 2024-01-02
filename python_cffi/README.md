# Python (CFFI)

> There are 3 alternative options available:
>
> using the newer [jsonrpc based bindings](../python_jsonrpc)
>
> frameworks:
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
pip3 install deltachat
```
If it doesn't work (for example because you are not on linux),
see https://py.delta.chat/cffi/install.html#installing-bindings-from-source for instructions on how to install it from source.


## Usage

```sh
python echo_bot.py /tmp/db --email ADDRESS --password PASSWORD
```

### References

- all available hooks can be found in https://github.com/deltachat/deltachat-core-rust/blob/master/python/src/deltachat/hookspec.py
- Python bindings documentation https://py.delta.chat
- https://py.delta.chat/cffi/examples.html
- source code of python bindings https://github.com/deltachat/deltachat-core-rust/tree/master/python
