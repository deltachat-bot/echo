# -*- coding: utf-8 -*-
from deltabot.hookspec import deltabot_hookimpl

@deltabot_hookimpl
def deltabot_init(bot):
    bot.filters.register(name=__name__, func=echo)


def echo(message, replies):
   """echo back text"""
   replies.add(message.text)