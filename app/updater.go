package app

import (
	"time"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"

	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type Updater struct {
	updater *ext.Updater
	bot     *Bot
	logger  *prelude.Logger
}

func NewUpdater(bot *Bot, dispatcher Dispatcher, logger *prelude.Logger) *Updater {
	result := &Updater{
		updater: ext.NewUpdater(dispatcher.GetBotDispatcher(), nil),
		bot:     bot,
		logger:  logger,
	}
	return result
}

func (updater *Updater) StartPolling() {
	err := updater.updater.StartPolling(updater.bot.GetBot(), &ext.PollingOpts{
		DropPendingUpdates: false,
		GetUpdatesOpts: &gotgbot.GetUpdatesOpts{
			Timeout: 30,
			RequestOpts: &gotgbot.RequestOpts{
				Timeout: time.Second * 30,
			},
		},
	})
	if err != nil {
		updater.logger.Error("error starting polling", "error", err)
		panic(err)
	}
}

func (updater *Updater) Idle() {
	updater.updater.Idle()
}
