package character

import (
	"encoding/json"

	ironsworncompanionbot "github.com/dc0d/IronswornCompanionBot"
	"github.com/dc0d/IronswornCompanionBot/core/model/oracles"
)

var (
	Tables struct{ Role, Goal, Descriptor, Disposition oracles.OracleTable }

	characterOracle oracles.Oracle
)

func init() {
	ironswornOraclesCharacterContent, err := ironsworncompanionbot.AssetsFS.ReadFile("assets/ironsworn_oracles_character.json")
	if err != nil {
		panic(err)
	}

	err = json.Unmarshal(ironswornOraclesCharacterContent, &characterOracle)
	if err != nil {
		panic(err)
	}

	for _, o := range characterOracle.Oracles {
		switch o.Name {
		case "Role":
			Tables.Role = o
		case "Goal":
			Tables.Goal = o
		case "Descriptor":
			Tables.Descriptor = o
		case "Disposition":
			Tables.Disposition = o
		}
	}
}
