package app

import (
	"fmt"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type CombatActionCommand struct {
	handlers.Command

	orcSvc model.OraclesService
	logger *prelude.Logger
}

func NewCombatActionCommand(orcSvc model.OraclesService, logger *prelude.Logger) ext.Handler {
	result := CombatActionCommand{
		orcSvc: orcSvc,
		logger: logger,
	}
	result.Command = handlers.NewCommand("combat_action", result.Execute)

	return result
}

func (cmd CombatActionCommand) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("combat_action command received", "RCVD", ctx.Message)

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, cmd.orcSvc.CombatAction(), nil)
	if err != nil {
		return fmt.Errorf("failed to send roll message: %w", err)
	}

	return nil
}
