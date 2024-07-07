package model

var _ Action[int, int] = ActionFunc[int, int](func(input int) (int, error) { panic("ignore this - must never happen") })
