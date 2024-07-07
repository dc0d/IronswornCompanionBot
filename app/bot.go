package app

import (
	gotgbot "github.com/PaulSonOfLars/gotgbot/v2"

	"github.com/dc0d/IronswornCompanionBot/config"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type Bot struct {
	*gotgbot.Bot

	logger *prelude.Logger
}

func NewBot(conf *config.Config, logger *prelude.Logger) *Bot {
	bot, err := gotgbot.NewBot(conf.Token, nil)
	if err != nil {
		panic(err)
	}

	result := Bot{Bot: bot, logger: logger}

	return &result
}

func (b *Bot) GetBot() *gotgbot.Bot {
	return b.Bot
}

func (b *Bot) SetCommands() {
	_, err := b.SetMyCommands([]gotgbot.BotCommand{
		{Command: "roll", Description: "Roll dice for Ironsworn"},
		{Command: "make_npc", Description: "Create a new NPC"},
		{Command: "ask_the_oracle", Description: "Ask the Oracle if something is true by choosing the odds in a situation"},
		{Command: "action_theme", Description: "Generate a narration theme with Action and Theme"},
		{Command: "aspect_focus", Description: "Generate a narration focus with Aspect and Focus"},
		{Command: "challenge_rank", Description: "Give a challenge rank"},
		{Command: "major_plot_twist", Description: "Give a major plot twist"},
		{Command: "mystic_backlash", Description: "Give a mystic backlash"},
		{Command: "combat_action", Description: "Give a combat action"},
		{Command: "start", Description: "Start the bot"},
	}, nil)
	if err != nil {
		b.logger.Error("failed to set commands", "error", err)
	}
}
