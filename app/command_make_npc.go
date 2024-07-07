package app

import (
	"fmt"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type MakeNPCCommand struct {
	handlers.Command

	orcSvc model.OraclesService
	logger *prelude.Logger
}

func NewMakeNPCCommand(orcSvc model.OraclesService, logger *prelude.Logger) ext.Handler {
	result := MakeNPCCommand{
		orcSvc: orcSvc,
		logger: logger,
	}
	result.Command = handlers.NewCommand("make_npc", result.Execute)

	return result
}

func (cmd MakeNPCCommand) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("make_npc command received", "RCVD", ctx.Message)

	npc := cmd.orcSvc.MakeNPC()

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, fmt.Sprintf("%v %v %v 🎯 %v", npc.Disposition, npc.Descriptor, npc.Role, npc.Goal), nil)
	if err != nil {
		return fmt.Errorf("failed to send roll message: %w", err)
	}

	return nil
}
