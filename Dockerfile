FROM zalando/rusty-team-build-base

ADD . /rusty-team
WORKDIR /rusty-team
RUN cargo build --release --target x86_64-unknown-linux-musl

WORKDIR /rusty-team/target/x86_64-unknown-linux-musl/release

ADD Dockerfile.final /rusty-team/target/x86_64-unknown-linux-musl/release/Dockerfile

CMD docker build -t danpersa/rusty-team:latest .
