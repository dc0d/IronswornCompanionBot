package model

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/suite"
)

type OddsTestSuite struct {
	suite.Suite
}

func (s *OddsTestSuite) TestValidateOdds() {
	var sut Odds = 1

	AlmostCertain.validateOdds()
	Likely.validateOdds()
	FiftyFifty.validateOdds()
	Unlikely.validateOdds()
	SmallChance.validateOdds()

	s.PanicsWithValue("invalid odds: 1", func() { sut.validateOdds() })
}

func (s *OddsTestSuite) TestStringerImplementation() {
	s.Equal("Almost Certain", AlmostCertain.String())
	s.Equal("Likely", Likely.String())
	s.Equal("50/50", FiftyFifty.String())
	s.Equal("Unlikely", Unlikely.String())
	s.Equal("Small Chance", SmallChance.String())

	s.PanicsWithValue("invalid odds: 0", func() { _ = Odds(0).String() })
}

func (s *OddsTestSuite) TestResolveOdds() {
	type testCase struct {
		n              int
		chosenOdds     Odds
		expectedOutput NoYes
	}
	cases := []testCase{
		{n: 1, chosenOdds: AlmostCertain, expectedOutput: false},
		{n: 10, chosenOdds: AlmostCertain, expectedOutput: false},
		{n: 11, chosenOdds: AlmostCertain, expectedOutput: true},
		{n: 25, chosenOdds: Likely, expectedOutput: false},
		{n: 26, chosenOdds: Likely, expectedOutput: true},
		{n: 50, chosenOdds: FiftyFifty, expectedOutput: false},
		{n: 51, chosenOdds: FiftyFifty, expectedOutput: true},
		{n: 75, chosenOdds: Unlikely, expectedOutput: false},
		{n: 76, chosenOdds: Unlikely, expectedOutput: true},
		{n: 100, chosenOdds: SmallChance, expectedOutput: true},
	}

	for _, c := range cases {
		s.Equal(c.expectedOutput, resolveOdds(c.n, c.chosenOdds), c)
	}
}

func (s *OddsTestSuite) TestParseOdds() {
	type testCase struct {
		input          string
		expectedOutput Odds
	}
	cases := []testCase{
		{input: "Almost Certain", expectedOutput: AlmostCertain},
		{input: "Likely", expectedOutput: Likely},
		{input: "50/50", expectedOutput: FiftyFifty},
		{input: "Unlikely", expectedOutput: Unlikely},
		{input: "Small Chance", expectedOutput: SmallChance},
	}

	for _, c := range cases {
		s.Equal(c.expectedOutput, parseOdds(c.input), c)
	}
}

func TestOddsTestSuite(t *testing.T) { suite.Run(t, new(OddsTestSuite)) }

var _ fmt.Stringer = Odds(0)
