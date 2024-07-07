package model

type Action[Input any, Output any] interface {
	Execute(Input) (Output, error)
}

type ActionFunc[Input any, Output any] func(Input) (Output, error)

func (f ActionFunc[Input, Output]) Execute(input Input) (Output, error) {
	return f(input)
}
