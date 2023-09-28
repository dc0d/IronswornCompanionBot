.PHONY: test
test:
	mix test --cover

format:
	mix format

lint:
	mix credo

lint_strict:
	mix credo --strict

analyze:
	mix dialyzer
