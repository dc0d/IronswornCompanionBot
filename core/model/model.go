package model

import (
	"math/rand/v2"
	"time"
)

type RandGenerator interface {
	Generate(n int) int
}

type Rand struct {
	rand *rand.Rand
}

func NewDefaultRand() *Rand {
	src := rand.NewPCG(uint64(time.Now().UnixMilli()), uint64(time.Now().Unix()))
	return &Rand{rand: rand.New(src)}
}

func NewCustomRand(rand *rand.Rand) *Rand {
	return &Rand{rand: rand}
}

func (r *Rand) Generate(n int) int {
	return r.rand.IntN(n)
}
