const { deltachat, log } = require('deltachat-node-bot-base')

// Start the deltachat core engine and handle incoming messages.
deltachat.start((chat, message) => {
  log("Got a message: ", message.getText())
  if (deltachat.getChatContacts(chat.getid()).length === 1) {
    // This is a 1-on-1 (aka "single") chat.
    deltachat.sendMessage(chat.getId(), message.getText())
  }
})
