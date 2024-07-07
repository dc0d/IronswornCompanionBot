package app

import (
	"fmt"

	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	handlers "github.com/PaulSonOfLars/gotgbot/v2/ext/handlers"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type ChallengeRankCommand struct {
	handlers.Command

	orcSvc model.OraclesService
	logger *prelude.Logger
}

func NewChallengeRankCommand(orcSvc model.OraclesService, logger *prelude.Logger) ext.Handler {
	result := ChallengeRankCommand{
		orcSvc: orcSvc,
		logger: logger,
	}
	result.Command = handlers.NewCommand("challenge_rank", result.Execute)

	return result
}

func (cmd ChallengeRankCommand) Execute(bot *gotgbot.Bot, ctx *ext.Context) error {
	cmd.logger.Info("challenge_rank command received", "RCVD", ctx.Message)

	challengeRank := cmd.orcSvc.ChallengeRank()

	switch challengeRank {
	case "Troublesome":
		challengeRank = challengeRank + " 🟢🟢🟢🟢🔴"
	case "Dangerous":
		challengeRank = challengeRank + " 🟢🟢🟢🔴🔴"
	case "Formidable":
		challengeRank = challengeRank + " 🟢🟢🔴🔴🔴"
	case "Extreme":
		challengeRank = challengeRank + " 🟢🔴🔴🔴🔴"
	case "Epic":
		challengeRank = challengeRank + " 🔴🔴🔴🔴🔴"
	default:
		challengeRank = challengeRank + " 👽"
	}

	_, err := bot.SendMessage(ctx.EffectiveChat.Id, challengeRank, nil)
	if err != nil {
		return fmt.Errorf("failed to send roll message: %w", err)
	}

	return nil
}
