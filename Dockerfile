FROM ubuntu:latest

WORKDIR /home/vilchain

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    curl \
    file \
    git

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustc --version && cargo --version

COPY ./chain .

ENV RUST_LOG="trace"

RUN cargo build --release

CMD ["cargo", "run", "--release"]