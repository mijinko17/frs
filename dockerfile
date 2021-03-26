FROM rust:1.50.0

WORKDIR /face-reader-server

# { Install dependencies.
RUN apt-get update
RUN apt-get -y install sqlite

RUN cargo install diesel_cli --no-default-features --features sqlite
# }

# { To reuse cache, build depencencies before copying our source.
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
RUN cargo install diesel_cli --no-default-features --features sqlite
# }

# { Copy files.
COPY ./src ./src
COPY ./migrations ./migrations
COPY .env .env
COPY diesel.toml diesel.toml
# }

# { Build our project.
RUN rm -f target/release/deps/face-reader-server*
RUN cargo build --release
RUN cargo install --path .
# }

# { Create tables.
RUN diesel migration run
# }

CMD ["face-reader-server"]