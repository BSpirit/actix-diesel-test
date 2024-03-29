FROM alpine:latest

WORKDIR /app/

COPY ./target/x86_64-unknown-linux-musl/release/main ./.env ./data.csv ./
COPY ./db ./db
COPY ./static ./static

EXPOSE 8088

CMD ["./main"]