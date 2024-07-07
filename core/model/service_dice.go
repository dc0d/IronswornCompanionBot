package model

type DiceService struct {
	rand RandGenerator
}

func NewDiceService(r RandGenerator) DiceService {
	return DiceService{
		rand: r,
	}
}

func (ds DiceService) RollD6() int {
	return ds.rand.Generate(6) + 1
}

func (ds DiceService) RollD10() int {
	return ds.rand.Generate(10) + 1
}

func (ds DiceService) RollD100() int {
	return ds.rand.Generate(100) + 1
}

func (ds DiceService) RollIronsworn() (result struct {
	D6  int
	D10 [2]int
}) {
	result.D6 = ds.RollD6()
	result.D10 = [2]int{ds.RollD10(), ds.RollD10()}
	return
}
