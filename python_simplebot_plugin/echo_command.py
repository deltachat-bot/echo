import simplebot
@simplebot.command
def echo(command, replies):
   """Echo back given text."""
   replies.add(text=command.payload or 'echo')