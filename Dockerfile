FROM rust:1.62-bullseye as builder

WORKDIR /app/
COPY . /app/
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN cargo install --path .

#FROM debian:bullseye
#WORKDIR /app/
#COPY --from=builder /usr/local/cargo/bin/tg_bot /usr/local/bin/tg_bot
#COPY --from=builder /app/urls_to_scrap.txt /app/urls_to_scrap.txt
#COPY --from=builder /app/.env /app/.env
RUN apt-get update
RUN apt-get install -y sqlite3 ca-certificates
