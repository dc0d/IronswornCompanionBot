package app

import (
	ext "github.com/PaulSonOfLars/gotgbot/v2/ext"
	fx "go.uber.org/fx"
	fxevent "go.uber.org/fx/fxevent"

	"github.com/dc0d/IronswornCompanionBot/config"
	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

func Run() {
	fx.New(
		fx.Provide(prelude.NewLogger),
		fx.WithLogger(func(logger *prelude.Logger) fxevent.Logger { return logger }),
		fx.Provide(config.NewConfig),
		fx.Provide(fx.Annotate(model.NewDefaultRand, fx.As(new(model.RandGenerator)))),
		fx.Provide(model.NewDiceService),
		fx.Provide(model.NewOraclesService),
		fx.Provide(
			AsHandler(NewStartCommand),
			AsHandler(NewRollCommand),
			AsHandler(NewAskTheOracleCommand),
			AsHandler(NewAskTheOracleCommandCallback),
			AsHandler(NewMakeNPCCommand),
			AsHandler(NewActionThemeCommand),
			AsHandler(NewAspectFocusCommand),
			AsHandler(NewChallengeRankCommand),
			AsHandler(NewMajorPlotTwistCommand),
			AsHandler(NewMysticBacklashCommand),
			AsHandler(NewCombatActionCommand),
		),
		fx.Provide(
			NewUpdater,
			fx.Annotate(
				NewDispatcher,
				fx.ParamTags(`group:"handlers"`),
			)),
		fx.Provide(NewBot),
		fx.Invoke(func(logger *prelude.Logger, updater *Updater, bot *Bot) {
			bot.SetCommands()

			updater.StartPolling()
			logger.Info("bot started")

			updater.Idle()
		}),
	)
}

func AsHandler(f any) any {
	return fx.Annotate(
		f,
		fx.As(new(ext.Handler)),
		fx.ResultTags(`group:"handlers"`),
	)
}
