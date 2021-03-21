# Python DeltaBot Plugin

This example is about creating a plugin for [deltabot](https://github.com/deltachat-bot/deltabot),
a pluggable deltachat bot.

> There are 2 alternative options availible:
>
> - [using simplebot](../python_simplebot_plugin) (simplebot is a maintained fork of deltabot with many plugins availible)
> - [using python directly](../python) (more controll, but also more code to take care of)

## Usage

### Setup deltabot

You need python 3.7 for this to work. (Newer versions could works, but this example was tested to work with this version, feel free to upgrade the example and drop us an pullrequest)

```sh
# Optional create virtualenv
pip install virtualenv
virtualenv .venv
source .venv/bin/activate
pip3 install -U pip wheel

# Install deltabot
pip install git+https://github.com/deltachat-bot/deltabot.git@ae298b414d0ed6b5ffbfcca466375f60e3a5b4b9 attrs

# init the bot (either set the env vars or replace them with the email credentials the bot should use)
deltabot --basedir=.data init "ADDR" "PASSWORD"
```

### Install the module/plugin

In deltabot you install python modules like this:

```sh
deltabot --basedir=.data add-module ./path/to/module.py
```

This folder contains two examples:

- `echo.py` - The clasic echo all text messages
- `echo_command.py` - An `/echo [text]` command

So the commands are the following:

```sh
deltabot --basedir=.data add-module echo.py
deltabot --basedir=.data add-module echo_command.py
```

To start type:

```
deltabot --basedir=.data serve
```

### Reference links

- https://github.com/deltachat-bot/deltabot/blob/master/examples/deltabot_echo/deltabot_echo.py
