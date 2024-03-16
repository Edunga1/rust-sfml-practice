# Only supports sfml 2.5.x because of the libsfml-dev package
FROM rust:1.76

RUN apt-get update && apt-get install -y libsfml-dev

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

CMD ["cargo", "run", "--release"]
