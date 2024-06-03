FROM rust:1.75.0

ENV CARGO_TARGET_DIR=/tmp/target \
    DEBIAN_FRONTEND=noninteractive 
    
RUN apt update 
RUN apt upgrade -y
RUN apt install -y -q \
    ca-certificates \
    locales \
    apt-transport-https\
    libssl-dev \
    libpq-dev \
    pkg-config \
    curl \
    build-essential \
    git \
    wget
RUN echo "install rust tools"
RUN rustup component add rustfmt
RUN cargo install cargo-watch cargo-make

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]