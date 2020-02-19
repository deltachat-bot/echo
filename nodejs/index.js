const { DeltaChat, C } = require('deltachat-node')
const path = require('path')

// Load config
var conf = require('rc')("dc-echo", {
    email_address: undefined,
    email_password: undefined,
})

// Setup DC
const dc = new DeltaChat()

function handleDCMessage(chatid, msgId) {
    const chat = dc.getChat(chatid)
    const msg = dc.getMessage(msgId)

    // only echo to DM
    if (chat.getType() === C.DC_CHAT_TYPE_SINGLE) {
        dc.sendMessage(chatid, msg.getText())
    }
}

dc.on('DC_EVENT_MSGS_CHANGED', (chatId, msgId) => {
    // Deaddrop fix for bot, otherwise first message would be ignored
    const message = dc.getMessage(msgId)
    if (message && message.isDeadDrop()) {
        handleDCMessage(dc.createChatByMessageId(msgId), msgId)
    }
})
dc.on('DC_EVENT_INCOMING_MSG', handleDCMessage)

// dc.on('ALL', console.log.bind(null, 'core |')) // advanced logging for debugging

// Start DC
dc.open(path.join(__dirname, 'deltachat-db'), () => {
    if (!dc.isConfigured()) {
        if (!conf.email_address || !conf.email_password) {
            console.error("Not configued and email address or password is missing.")
            process.exit(1)
        }        
        dc.configure({
            addr: conf.email_address,
            mail_pw: conf.email_password,
            e2ee_enabled: true,
        }, () => {
            console.log('init done')
        })
    } else {
        console.log('init done')
    }
})

process.on('exit', () => {
    dc.close(() => {
        // clean up, save state or close database connections
    })
})
