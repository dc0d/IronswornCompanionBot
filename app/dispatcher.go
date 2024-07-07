package app

import (
	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"

	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type Dispatcher struct {
	dispatcher *ext.Dispatcher
}

func NewDispatcher(handlers []ext.Handler, logger *prelude.Logger) Dispatcher {
	dispatcher := ext.NewDispatcher(&ext.DispatcherOpts{
		// If an error is returned by a handler, log it and continue going.
		Error: func(b *gotgbot.Bot, ctx *ext.Context, err error) ext.DispatcherAction {
			logger.Error("an error occurred while handling update:", "error", err)
			return ext.DispatcherActionNoop
		},
		MaxRoutines: ext.DefaultMaxRoutines,
	})

	for _, h := range handlers {
		dispatcher.AddHandler(h)
	}

	result := Dispatcher{dispatcher: dispatcher}

	return result
}

func (dispatcher *Dispatcher) GetBotDispatcher() *ext.Dispatcher {
	return dispatcher.dispatcher
}
