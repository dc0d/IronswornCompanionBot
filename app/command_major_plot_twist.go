package app

import (
	"fmt"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type MajorPlotTwistCommand struct {
	handlers.Command

	orcSvc model.OraclesService
	logger *prelude.Logger
}

func NewMajorPlotTwistCommand(orcSvc model.OraclesService, logger *prelude.Logger) ext.Handler {
	result := MajorPlotTwistCommand{
		orcSvc: orcSvc,
		logger: logger,
	}
	result.Command = handlers.NewCommand("major_plot_twist", result.Execute)

	return result
}

func (cmd MajorPlotTwistCommand) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("major_plot_twist command received", "RCVD", ctx.Message)

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, cmd.orcSvc.MajorPlotTwist(), nil)
	if err != nil {
		return fmt.Errorf("failed to send roll message: %w", err)
	}

	return nil
}
