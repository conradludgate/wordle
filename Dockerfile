FROM docker.io/rust:1.58-slim-bullseye

WORKDIR /usr/src/wordle
COPY . .

RUN cargo install --path .

FROM docker.io/debian:bullseye-slim
COPY --from=0 /usr/local/cargo/bin/wordle /usr/local/bin/wordle

CMD /usr/local/bin/wordle
