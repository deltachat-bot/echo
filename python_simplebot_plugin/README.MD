# Python Simplebot Plugin

This example is about creating a plugin for [simplebot](https://github.com/simplebot-org/simplebot),
a pluggable deltachat bot,
which is a fork of [deltabot](https://github.com/deltachat-bot/deltabot) and has a [vast ecosystem of plugins](https://pypi.org/search/?q=simplebot&o=&c=Environment+%3A%3A+Plugins) already availible and used in production.

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

# Install simplebot
pip3 install simplebot
# or alternativly: pip3 install git+https://github.com/simplebot-org/simplebot/

# init the bot (either set the env vars or replace them with the email credentials the bot should use)
simplebot --account ./.simplebotdata init $ADDR "$PASSWORD"
```

### Install the plugin

In simplebot you install plugins like this:

```sh
# with a default account set
simplebot plugin --add /path/to/plugin.py
# with account
simplebot --account bot@example.org plugin --add /path/to/plugin.py
# with account folder
simplebot --account ./.simplebotdata plugin --add /path/to/plugin.py
```

This folder contains two examples:

- `echo.py` - The clasic echo all text messages
- `echo_command.py` - An `/echo [text]` command

So the commands are the following:

```sh
simplebot --account ./.simplebotdata plugin --add echo.py
simplebot --account ./.simplebotdata plugin --add echo_command.py
```

To start type:

```
simplebot -a ./.simplebotdata serve
```

## References

Also see the source of the existing plugins over at https://github.com/simplebot-org/.

> **Tip:** instead of manually registering script by script, simplebot supports registering folders (and also python package folders), ex. `~/my_plugins` then all script you add there will be loaded when the bot starts and you can add new script or remove existing scripts without having to add/remove using these commands
