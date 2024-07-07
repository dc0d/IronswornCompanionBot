package prompts

import (
	"encoding/json"

	ironsworncompanionbot "github.com/dc0d/IronswornCompanionBot"
	"github.com/dc0d/IronswornCompanionBot/core/model/oracles"
)

var (
	// Feature contains Aspect and Focus

	Tables struct{ Action, Theme, Aspect, Focus oracles.OracleTable }

	promptsOracle oracles.Oracle
)

func init() {
	content, err := ironsworncompanionbot.AssetsFS.ReadFile("assets/ironsworn_oracles_prompts.json")
	if err != nil {
		panic(err)
	}

	err = json.Unmarshal(content, &promptsOracle)
	if err != nil {
		panic(err)
	}

	var oracleFeature oracles.OracleTable

	for _, o := range promptsOracle.Oracles {
		switch o.Name {
		case "Action":
			Tables.Action = o
		case "Theme":
			Tables.Theme = o
		case "Feature":
			oracleFeature = o
		}
	}

	for _, o := range oracleFeature.Oracles {
		switch o.Name {
		case "Aspect":
			Tables.Aspect = o
		case "Focus":
			Tables.Focus = o
		}
	}
}
