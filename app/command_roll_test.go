package app

import (
	"testing"

	"github.com/stretchr/testify/suite"

	"github.com/dc0d/IronswornCompanionBot/core/model"
	"github.com/dc0d/IronswornCompanionBot/prelude"
)

type RollCommandTestSuite struct {
	suite.Suite

	sut RollCommand
}

func (s *RollCommandTestSuite) SetupTest() {
	dice := model.NewDiceService(NewFakeRand())
	s.sut = (NewRollCommand(dice, prelude.NewLogger())).(RollCommand)
}

func (s *RollCommandTestSuite) TestRollAction() {
	type testCase struct {
		expectedOutput string
	}
	cases := []testCase{
		{expectedOutput: "5 🎲 4 - 7"},
		{expectedOutput: "6 🎲 7 - 7 🎁"},
		{expectedOutput: "5 🎲 4 - 1 💪"},
		{expectedOutput: "6 🎲 3 - 3 🎁 💪"},
	}

	for _, tc := range cases {
		output := s.sut.rollIronsworn()

		s.Equal(tc.expectedOutput, string(output))
	}
}

func TestRollCommand(t *testing.T) { suite.Run(t, new(RollCommandTestSuite)) }

//

type FakeRand struct {
	stream []int
}

func NewFakeRand() *FakeRand {
	stream := []int{
		5, 4, 7,
		6, 7, 7,
		5, 4, 1,
		6, 3, 3,
	}
	for k, v := range stream {
		stream[k] = v - 1
	}
	return &FakeRand{
		stream: stream,
	}
}

func (f *FakeRand) Generate(n int) int {
	result := f.stream[0]
	f.stream = f.stream[1:]
	return result
}

var _ model.RandGenerator = &FakeRand{}
