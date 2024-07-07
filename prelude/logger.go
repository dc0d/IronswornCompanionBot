package prelude

import (
	"log/slog"
	"os"

	"go.uber.org/fx/fxevent"
)

type Logger struct {
	*slog.Logger
}

func NewLogger() *Logger {
	logger := slog.New(
		slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{
			AddSource: true,
			Level:     slog.LevelDebug,
		}))

	slog.SetDefault(logger)

	result := &Logger{Logger: logger}

	return result
}

func (l *Logger) LogEvent(e fxevent.Event) {
	l.Info("fx event", "event", e)
}
