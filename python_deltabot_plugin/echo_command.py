# -*- coding: utf-8 -*-
from deltabot.hookspec import deltabot_hookimpl

version = '0.5'

@deltabot_hookimpl
def deltabot_init(bot):
    bot.commands.register(name="/echo", func=process_command_echo)


def process_command_echo(command, replies):
    """ Echoes back received message.
    To use it you can simply send a message starting with
    the command '/echo'. Example: `/echo hello world`
    """
    message = command.message
    contact = message.get_sender_contact()
    sender = 'From: {} <{}>'.format(contact.display_name, contact.addr)
    replies.add(text="{}\n{!r}".format(sender, command.payload))
