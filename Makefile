TARGET_OS=linux
TARGET_ARCH=amd64
GF_GOMEMLIMIT=150MiB
GO_VERSION=1.22

.PHONY: test
test:
	go test -timeout 30s -cover -v -count 1 ./...

race:
	go test -count=1 -race -timeout 180s ./...

lint:
	golangci-lint run ./...

vulncheck:
	govulncheck ./...

gen:
	GOFLAGS=-mod=mod go generate ./...

run:
	cd ./cmd/companion && go build && ./companion

tidy:
	go mod tidy

docker-build:
	docker build --tag ironsworn-companion-img \
		--progress=plain --no-cache \
		--build-arg ICB_TOKEN=$(ICB_TOKEN) \
		--build-arg TARGET_OS=$(TARGET_OS) \
		--build-arg TARGET_ARCH=$(TARGET_ARCH) \
		--build-arg GF_GOMEMLIMIT=$(GF_GOMEMLIMIT) \
		--build-arg GO_VERSION=$(GO_VERSION) \
		.

docker-run:
	docker run -i --rm --env "ICB_TOKEN=$(ICB_TOKEN)" ironsworn-companion-img
