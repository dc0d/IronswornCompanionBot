package turningpoint

import (
	"encoding/json"

	ironsworncompanionbot "github.com/dc0d/IronswornCompanionBot"
	"github.com/dc0d/IronswornCompanionBot/core/model/oracles"
)

var (
	Tables struct{ ChallengeRank, CombatAction, MajorPlotTwist, MysticBacklash oracles.OracleTable }

	turningpointOracle oracles.Oracle
)

func init() {
	content, err := ironsworncompanionbot.AssetsFS.ReadFile("assets/ironsworn_oracles_turning_point.json")
	if err != nil {
		panic(err)
	}

	err = json.Unmarshal(content, &turningpointOracle)
	if err != nil {
		panic(err)
	}

	for _, o := range turningpointOracle.Oracles {
		switch o.Name {
		case "Challenge Rank":
			Tables.ChallengeRank = o
		case "Combat Action":
			Tables.CombatAction = o
		case "Major Plot Twist":
			Tables.MajorPlotTwist = o
		case "Mystic Backlash":
			Tables.MysticBacklash = o
		}
	}
}
