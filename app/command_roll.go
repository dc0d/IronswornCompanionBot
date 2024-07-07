package app

import (
	"fmt"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type RollCommand struct {
	handlers.Command
	dice   model.DiceService
	logger *prelude.Logger
}

func NewRollCommand(dice model.DiceService, logger *prelude.Logger) ext.Handler {
	result := RollCommand{
		dice:   dice,
		logger: logger,
	}
	result.Command = handlers.NewCommand("roll", result.Execute)

	return result
}

func (cmd RollCommand) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("roll command received", "RCVD", ctx.Message)

	result := cmd.rollIronsworn()

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, string(result), nil)
	if err != nil {
		return fmt.Errorf("failed to send roll message: %w", err)
	}

	return nil
}

func (cmd RollCommand) rollIronsworn() string {
	rolled := cmd.dice.RollIronsworn()

	challenged := fmt.Sprintf("%v - %v", rolled.D10[0], rolled.D10[1])
	if rolled.D10[0] == rolled.D10[1] {
		challenged += " 🎁"
	}
	if rolled.D6 > rolled.D10[0] && rolled.D6 > rolled.D10[1] {
		challenged += " 💪"
	}

	return fmt.Sprintf("%v 🎲 %v", rolled.D6, challenged)
}
