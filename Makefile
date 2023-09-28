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

docker-build:
	docker build --tag ironsworn-companion-img \
		--progress=plain --no-cache \
		--build-arg ICB_TOKEN=$(ICB_TOKEN) \
		.

docker-run:
	docker run -i --rm --env "ICB_TOKEN=$(ICB_TOKEN)" ironsworn-companion-img
