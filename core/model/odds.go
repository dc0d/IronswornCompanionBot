package model

import "fmt"

/*
Odds 			The answer is ‘yes’ if you roll...
---------------|----------------------------------
Almost Certain	11 or greater
Likely 			26 or greater
50/50 			51 or greater
Unlikely 		76 or greater
Small Chance 	91 or greater
*/

func ResolveOdds(n int, chosenOdds string) NoYes {
	parsedOdds := parseOdds(chosenOdds)
	return resolveOdds(n, parsedOdds)
}

type NoYes bool

func (n NoYes) String() string {
	if n {
		return "Yes"
	}
	return "No"
}

// Odds represents the odds of a roll. It does not participate in calculations.
// It can have only one of the valid values.
type Odds int

func (o Odds) validateOdds() {
	switch o {
	case AlmostCertain:
		return
	case Likely:
		return
	case FiftyFifty:
		return
	case Unlikely:
		return
	case SmallChance:
		return
	}

	panic(fmt.Sprintf("invalid odds: %v", int(o)))
}

func (o Odds) String() string {
	switch o {
	case AlmostCertain:
		return "Almost Certain"
	case Likely:
		return "Likely"
	case FiftyFifty:
		return "50/50"
	case Unlikely:
		return "Unlikely"
	case SmallChance:
		return "Small Chance"
	}

	panic(fmt.Sprintf("invalid odds: %v", int(o)))
}

const (
	AlmostCertain Odds = 11
	Likely        Odds = 26
	FiftyFifty    Odds = 51
	Unlikely      Odds = 76
	SmallChance   Odds = 91
)

func parseOdds(s string) Odds {
	switch s {
	case "Almost Certain":
		return AlmostCertain
	case "Likely":
		return Likely
	case "50/50":
		return FiftyFifty
	case "Unlikely":
		return Unlikely
	case "Small Chance":
		return SmallChance
	}

	panic(fmt.Sprintf("invalid odds string: %s", s))
}

// resolveOdds
// Input: 1-100 and the chosed odds
// Output: NoYes
func resolveOdds(n int, chosenOdds Odds) NoYes {
	return n >= int(chosenOdds)
}
