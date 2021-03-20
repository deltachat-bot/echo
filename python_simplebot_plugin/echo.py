import simplebot
@simplebot.filter
def echo(message, replies):
   """Echo back messages."""
   replies.add(text=message.text)