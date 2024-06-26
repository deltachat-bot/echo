# Python deltabot-cli

This example is about creating a bot with the bot framework [deltabot-cli-py](https://github.com/deltachat-bot/deltabot-cli-py/), which gives a hook based interface and a cli interface to configure your bot.

> This is a python framework for bots, see [../README.md](../README.md) for other approaches to write bot in python.

## Usage

> Per default the bot data and configuration is stored in your user directory
> (`Library/Application Support/bot_name/`, `.config/bot_name/`, on windows probably in `%APPDATA%` <!-- todo the location on windows needs to be checked -->).
>
> You can change configuration location with `--config-dir` flag (`--config-dir PATH` or `-c PATH`).

### Setup

#### Install

```sh
# Optional create virtualenv
pip install virtualenv
virtualenv .venv
source .venv/bin/activate

# install rpc server
pip install deltachat-rpc-server

pip install deltabot-cli-py
```

#### Configure

```sh
python ./echobot.py init address@example.com password
```


### Start

```sh
python ./echobot.py serve
```
