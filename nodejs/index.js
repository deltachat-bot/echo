//@ts-check
const { Context, C } = require("deltachat-node");
const path = require("path");

// Load config
var conf = require("rc")("dc-echo", {
  email_address: undefined,
  email_password: undefined,
});

// Setup DC
const dc = Context.open(path.join(__dirname, "deltachat-db"));

function handleDCMessage(chatid, msgId) {
  const chat = dc.getChat(chatid);
  console.log(chat.isContactRequest());
  const msg = dc.getMessage(msgId);

  if (chat.isContactRequest()) {
    dc.acceptChat(chatid);
  }

  // only echo to DM
  if (chat.getType() === C.DC_CHAT_TYPE_SINGLE) {
    dc.sendMessage(chatid, msg.getText());
  }
}

dc.on("DC_EVENT_INCOMING_MSG", handleDCMessage);

//dc.on("ALL", console.log.bind(null, "core |")); // advanced logging for debugging

// Start DC
async function setup() {
  if (!dc.isConfigured()) {
    console.log("not configured, doing that now");
    if (!conf.email_address || !conf.email_password) {
      console.error("Not configued and email address or password is missing.");
      process.exit(1);
    }
    await dc.configure({
      addr: conf.email_address,
      mail_pw: conf.email_password,
      e2ee_enabled: true,
      // the bot flag is only respected core version >= 47, but it does not hurt to already add it
      bot: true,
    });
    console.log("config done");
  }
  dc.startIO();
  console.log("init done");
}

setup();

process.on("exit", () => {
  dc.stopIO();
});
