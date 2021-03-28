import simplebot
@simplebot.command
def echo(payload, replies):
   """Echo back given text."""
   replies.add(text=payload or 'echo')
