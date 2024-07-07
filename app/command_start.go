package app

import (
	"fmt"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"

	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type StartCommand struct {
	handlers.Command
	logger *prelude.Logger
}

func NewStartCommand(logger *prelude.Logger) ext.Handler {
	result := StartCommand{
		logger: logger,
	}
	result.Command = handlers.NewCommand("start", result.Execute)

	return result
}

func (cmd StartCommand) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("start command received", "chat", ctx.EffectiveChat)

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, startReply, nil)
	if err != nil {
		return fmt.Errorf("failed to send start message: %w", err)
	}

	return nil
}

const startReply = `
Let's get on with our journey! 🎲

This bot is designed to help you play Ironsworn, a free tabletop role-playing game by Shawn Tomkin. It can help you with dice rolls, generate NPCs, and answer questions using the oracles.

You can choose a command from the menu or type a command directly. Also, typing '/' brings up the command menu.
`
