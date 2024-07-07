package app

import (
	"fmt"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type MysticBacklashCommand struct {
	handlers.Command

	orcSvc model.OraclesService
	logger *prelude.Logger
}

func NewMysticBacklashCommand(orcSvc model.OraclesService, logger *prelude.Logger) ext.Handler {
	result := MysticBacklashCommand{
		orcSvc: orcSvc,
		logger: logger,
	}
	result.Command = handlers.NewCommand("mystic_backlash", result.Execute)

	return result
}

func (cmd MysticBacklashCommand) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("mystic_backlash command received", "RCVD", ctx.Message)

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, cmd.orcSvc.MysticBacklash(), nil)
	if err != nil {
		return fmt.Errorf("failed to send roll message: %w", err)
	}

	return nil
}
