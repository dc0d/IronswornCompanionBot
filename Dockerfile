ARG TARGET_OS
ARG TARGET_ARCH
ARG GF_GOMEMLIMIT
ARG GO_VERSION
ARG ICB_TOKEN

# << builder >>
FROM golang:${GO_VERSION}-bookworm as builder

WORKDIR /app
COPY ./ ./
RUN go mod tidy
RUN cd ./cmd/companion/ && GOOS=${TARGET_OS} GOARCH=${TARGET_ARCH} GOMEMLIMIT=${GF_GOMEMLIMIT} CGO_ENABLED=0 GCCGO=75 go build

# << app >>
FROM debian:bookworm

RUN apt-get update
RUN apt-get install ca-certificates -y

WORKDIR /app
COPY --from=builder /app/cmd/companion/companion /app/companion

ENV ICB_TOKEN=$ICB_TOKEN
EXPOSE 8081/tcp 8082/tcp

CMD ["/app/companion"]
