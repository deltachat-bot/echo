#!/usr/bin/env python3
"""Minimal echo-bot example."""

from deltabot_cli import BotCli
from deltachat2 import MsgData, events

cli = BotCli("echobot")


@cli.on(events.RawEvent)
def log_event(bot, accid, event):
    bot.logger.debug(f"[acc={accid}] {event}")


@cli.on(events.NewMessage)
def echo(bot, accid, event):
    msg = event.msg
    reply = MsgData(text=msg.text)
    bot.rpc.send_msg(accid, msg.chat_id, reply)


if __name__ == "__main__":
    cli.start()
