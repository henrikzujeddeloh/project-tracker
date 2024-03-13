FROM rust:latest as build

RUN cargo install sqlx-cli

# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs

WORKDIR /usr/src/project-tracker

COPY Cargo.toml Cargo.lock ./

# Build dependencies (this step might change infrequently, so it's done first to leverage Docker's layer caching)
# RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

COPY . .

# Install npm dependencies from package.json
RUN npm install

# Build the CSS using Tailwind
RUN npx tailwindcss -i ./css/input.css -o ./css/output.css --minify

# Assuming you want to copy Font Awesome to your assets directory
# RUN cp -r node_modules/@fortawesome/fontawesome-free/webfonts ./assets/
# RUN cp -r node_modules/@fortawesome/fontawesome-free/css ./assets/

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
COPY js/ ./js/
# Ensure you copy the built CSS, not the source
COPY --from=build /usr/src/project-tracker/css/output.css ./css/
# Copy the Font Awesome fonts and CSS
COPY --from=build /usr/src/project-tracker/node_modules/@fortawesome/fontawesome-free/webfonts ./assets/webfonts/
COPY --from=build /usr/src/project-tracker/node_modules/@fortawesome/fontawesome-free/css ./assets/css/

EXPOSE 4200

COPY entrypoint.sh .
RUN chmod +x entrypoint.sh
ENTRYPOINT ["./entrypoint.sh"]
