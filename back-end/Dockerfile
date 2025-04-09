FROM golang:latest

WORKDIR /src

COPY go.mod go.sum ./

RUN go mod download

COPY src ./src

RUN go build -o main ./src/main.go

EXPOSE 8080
CMD ["./main"]
