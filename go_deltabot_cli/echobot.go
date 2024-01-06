package main

import (
	"github.com/deltachat-bot/deltabot-cli-go/botcli"
	"github.com/deltachat/deltachat-rpc-client-go/deltachat"
	"github.com/spf13/cobra"
)

func main() {
	cli := botcli.New("echobot")

	// incoming message handling
	cli.OnBotInit(func(cli *botcli.BotCli, bot *deltachat.Bot, cmd *cobra.Command, args []string) {
		bot.OnNewMsg(func(bot *deltachat.Bot, accId deltachat.AccountId, msgId deltachat.MsgId) {
			msg, _ := bot.Rpc.GetMessage(accId, msgId)
			if msg.FromId > deltachat.ContactLastSpecial && msg.Text != "" {
				bot.Rpc.MiscSendTextMessage(accId, msg.ChatId, msg.Text)
			}
		})
	})
	cli.OnBotStart(func(cli *botcli.BotCli, bot *deltachat.Bot, cmd *cobra.Command, args []string) {
		cli.Logger.Info("OnBotStart event triggered: bot is about to start!")
	})
	cli.Start()
}
