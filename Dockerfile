FROM rust:1.44-alpine

COPY . .

RUN cargo install --path .  --root /usr

EXPOSE 8080

CMD ["/usr/bin/rust-string-outputter"]
