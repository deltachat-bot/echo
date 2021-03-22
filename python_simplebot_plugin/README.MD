# Python Simplebot Plugin

This example is about creating a plugin for [simplebot](https://github.com/SimpleBot-Inc/simplebot),
a pluggable deltachat bot,
which is a fork of [deltabot](https://github.com/deltachat-bot/deltabot) and has a [vast ecosystem of plugins](https://github.com/SimpleBot-Inc/simplebot_plugins) already availible and used in production.

> There are 2 alternative options available:
>
> - [using deltabot](../python_deltabot_plugin) (similar to simplebot, but less plugins)
> - [using python directly](../python) (more control, but also more code to take care of)

## Usage

### Setup simplebot

You need python 3.7 for this to work. (Newer versions could work, but this example was tested to work with this version, feel free to upgrade the example and drop us a pull request)

```sh
# Optional create virtualenv
pip install virtualenv
virtualenv .venv
source .venv/bin/activate
pip3 install -U pip wheel

# install deltachat
pip3 install --pre -U -i https://m.devpi.net/dc/master deltachat # if it doesn't work, see https://github.com/deltachat/deltachat-core-rust/tree/master/python for instructions on how to install it from source)

# Install simplebot
pip3 install https://github.com/simplebot-inc/simplebot/archive/master.zip
# or alternativly pip3 install simplebot

# init the bot (either set the env vars or replace them with the email credentials the bot should use)
simplebot --basedir ./.simplebotdata init $ADDR "$PASSWORD"
```

### Install the plugin

In simple bot you install plugins like this:

```sh
simplebot plugin --add /path/to/plugin.py
```

This folder contains two examples:

- `echo.py` - The clasic echo all text messages
- `echo_command.py` - An `/echo [text]` command

So the commands are the following:

```sh
simplebot --basedir ./.simplebotdata --add echo.py
simplebot --basedir ./.simplebotdata --add echo_command.py
```

To start type:

```
simplebot --basedir ./.simplebotdata serve
```

## References

Also see the source of the existing plugins over at https://github.com/SimpleBot-Inc/simplebot_plugins.
