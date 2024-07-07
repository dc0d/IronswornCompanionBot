package config

import "os"

type Config struct {
	Token string
}

func NewConfig() *Config {
	token := os.Getenv("ICB_TOKEN")
	if token == "" {
		panic("TOKEN environment variable is empty")
	}

	return &Config{
		Token: token,
	}
}
