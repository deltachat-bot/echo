//@ts-check

import { startDeltaChat } from "@deltachat/stdio-rpc-server";
import { C } from "@deltachat/jsonrpc-client";

async function main() {
  const dc = await startDeltaChat("deltachat-data");
  console.log("Using deltachat-rpc-server at " + dc.pathToServerBinary);

  // log all events to console
  // dc.on("ALL", console.debug.bind("[core]"));

  // or only log what you want
  dc.on("Info", (accountId, { msg }) =>
    console.info(accountId, "[core:info]", msg)
  );
  dc.on("Warning", (accountId, { msg }) =>
    console.warn(accountId, "[core:warn]", msg)
  );
  dc.on("Error", (accountId, { msg }) =>
    console.error(accountId, "[core:error]", msg)
  );

  let firstAccount = (await dc.rpc.getAllAccounts())[0];
  if (!firstAccount) {
    firstAccount = await dc.rpc.getAccountInfo(await dc.rpc.addAccount());
  }
  if (firstAccount.kind === "Unconfigured") {
    console.info("account not configured, trying to login now...");
    try {
      const addr = Deno.env.get("ADDR");
      const mail_pw = Deno.env.get("MAIL_PW");
      const chatmail_qr = Deno.env.get("CHATMAIL_QR");
      if (addr && mail_pw) {
        await dc.rpc.batchSetConfig(firstAccount.id, {
          addr,
          mail_pw,
        });
      } else if (chatmail_qr) {
        await dc.rpc.setConfigFromQr(firstAccount.id, chatmail_qr);
      } else {
        throw new Error(
          "Credentials missing, you need to set ADDR and MAIL_PW, or CHATMAIL_QR"
        );
      }
      await dc.rpc.batchSetConfig(firstAccount.id, {
        bot: "1",
        e2ee_enabled: "1",
      });
      await dc.rpc.configure(firstAccount.id);
    } catch (error) {
      console.error("Could not log in to account:", error);
      Deno.exit(1);
    }
  } else {
    await dc.rpc.startIo(firstAccount.id);
  }

  const botAccountId = firstAccount.id;
  const emitter = dc.getContextEvents(botAccountId);
  emitter.on("IncomingMsg", async ({ chatId, msgId }) => {
    const chat = await dc.rpc.getBasicChatInfo(botAccountId, chatId);
    // only echo to DM chat
    if (chat.chatType === C.DC_CHAT_TYPE_SINGLE) {
      const message = await dc.rpc.getMessage(botAccountId, msgId);
      await dc.rpc.miscSendTextMessage(
        botAccountId,
        chatId,
        message.text || ""
      );
    }
  });

  const botAddress = await dc.rpc.getConfig(botAccountId, "addr");
  const verificationQRCode = (
    await dc.rpc.getChatSecurejoinQrCodeSvg(botAccountId, null)
  )[0];
  console.info("".padEnd(40, "="));
  console.info("The email address of your bot is: ", botAddress);
  console.info(
    `Verify Bot contact (if you use chatmail this is nessesary to contact the bot from outside the chatmail instance that the bot uses):
copy this code and \"scan\" it with delta chat:

${verificationQRCode}`
  );
  console.info("".padEnd(40, "="));
}

main();
