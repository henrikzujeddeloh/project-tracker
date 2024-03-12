FROM rust:latest as build

RUN cargo install sqlx-cli

WORKDIR /usr/src/project-tracker

COPY Cargo.toml Cargo.lock ./

# Build dependencies (this step might change infrequently, so it's done first to leverage Docker's layer caching)
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

COPY . .

RUN cargo build --release

FROM ubuntu:24.04

RUN apt-get update \
&& apt-get install -y --no-install-recommends tzdata \
&& apt-get clean \
&& apt-get autoremove -y \
&& apt-get purge -y --auto-remove \
&& rm -rf /var/lib/ap/lists/* /tmp/* /var/tmp/*

WORKDIR /usr/src/project-tracker

COPY --from=build /usr/src/project-tracker/target/release/project-tracker .
COPY --from=build /usr/local/cargo/bin/sqlx /usr/local/bin/

COPY migrations/ ./migrations/
COPY css/ ./css
COPY assets/ ./assets

EXPOSE 4200

COPY entrypoint.sh .
RUN chmod +x entrypoint.sh
ENTRYPOINT ["./entrypoint.sh"]
