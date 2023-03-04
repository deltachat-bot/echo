package main

import (
	"log"
	"os"

	"github.com/deltachat/deltachat-rpc-client-go/deltachat"
)

func logEvent(event *deltachat.Event) {
	log.Printf("%v: %v", event.Type, event.Msg)
}

func main() {
	rpc := deltachat.NewRpcIO()
	defer rpc.Stop()
	rpc.Start()

	manager := &deltachat.AccountManager{rpc}
	sysinfo, _ := manager.SystemInfo()
	log.Println("Running deltachat core", sysinfo["deltachat_core_version"])

	bot := deltachat.NewBotFromAccountManager(manager)
	bot.On(deltachat.EVENT_INFO, logEvent)
	bot.On(deltachat.EVENT_WARNING, logEvent)
	bot.On(deltachat.EVENT_ERROR, logEvent)
	bot.OnNewMsg(func(msg *deltachat.Message) {
		snapshot, _ := msg.Snapshot()
		chat := deltachat.Chat{bot.Account, snapshot.ChatId}
		chat.SendText(snapshot.Text)
	})

	if !bot.IsConfigured() {
		log.Println("Bot not configured, configuring...")
		err := bot.Configure(os.Args[1], os.Args[2])
		if err != nil {
			log.Fatalln(err)
		}
	}

	addr, _ := bot.GetConfig("addr")
	log.Println("Listening at:", addr)
	bot.RunForever()
}
