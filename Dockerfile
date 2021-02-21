FROM rust:1.50

RUN apt-get update && apt-get install -y \
    python3-dev \
    python-dev

WORKDIR /usr/src/app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release

RUN ln -s target/release/libwordsearch.so wordsearch.so

CMD [ "python3", "./main.py" ]