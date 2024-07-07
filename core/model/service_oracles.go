package model

import (
	"github.com/dc0d/IronswornCompanionBot/core/model/oracles"
	"github.com/dc0d/IronswornCompanionBot/core/model/oracles/character"
	"github.com/dc0d/IronswornCompanionBot/core/model/oracles/prompts"
	turningpoint "github.com/dc0d/IronswornCompanionBot/core/model/oracles/turning_point"
)

type OraclesService struct {
	dice DiceService
}

func NewOraclesService(dice DiceService) OraclesService {
	return OraclesService{
		dice: dice,
	}
}

func (os OraclesService) MakeNPC() (result struct{ Role, Goal, Descriptor, Disposition string }) {
	result.Role = os.findChoice(character.Tables.Role)
	result.Goal = os.findChoice(character.Tables.Goal)
	result.Descriptor = os.findChoice(character.Tables.Descriptor)
	result.Disposition = os.findChoice(character.Tables.Disposition)
	return
}

func (os OraclesService) ActionTheme() (result struct{ Action, Theme string }) {
	result.Action = os.findChoice(prompts.Tables.Action)
	result.Theme = os.findChoice(prompts.Tables.Theme)
	return
}

func (os OraclesService) AspectFocus() (result struct{ Aspect, Focus string }) {
	result.Aspect = os.findChoice(prompts.Tables.Aspect)
	result.Focus = os.findChoice(prompts.Tables.Focus)
	return
}

func (os OraclesService) ChallengeRank() string {
	return os.findChoice(turningpoint.Tables.ChallengeRank)
}

func (os OraclesService) MajorPlotTwist() string {
	return os.findChoice(turningpoint.Tables.MajorPlotTwist)
}

func (os OraclesService) MysticBacklash() string {
	return os.findChoice(turningpoint.Tables.MysticBacklash)
}

func (os OraclesService) CombatAction() string {
	return os.findChoice(turningpoint.Tables.CombatAction)
}

func (os OraclesService) findChoice(table oracles.OracleTable) string {
	rolled := os.dice.RollD100()
	for _, record := range table.OracleTable {
		if record.Chance >= rolled {
			return record.Description
		}
	}

	panic("no choice found - we should not be here!")
}
