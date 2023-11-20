FROM ubuntu:latest

WORKDIR /home/vilchain

RUN set -xe; \
    apt-get update && \
    apt-get install -y \
    build-essential \
    curl \
    file \
    git 

# --- Setting up Nodejs in the docker image ---
ENV NODE_VERSION=21.1.0

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

ENV NVM_DIR=/root/.nvm

RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}

RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}

RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}

ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"

RUN node --version

RUN npm --version

RUN npm install --global yarn
# --- Setting up Nodejs in the docker image ---


# --- Setting up Rust in the docker image ---
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustc --version && cargo --version
# --- Setting up Rust in the docker image ---

# --- Setting up Project and running it in the docker image ---
COPY ./chain .

RUN cargo build --release

RUN echo "$NODE_PORT"

CMD ["cargo", "run", "--release"]
# --- Setting up Project and running it in the docker image ---
