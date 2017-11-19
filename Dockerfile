FROM rustlang/rust:nightly
WORKDIR /usr/src/myapp
RUN apt-get update && \
       apt-get install -y \
       libpq5 \
       --no-install-recommends
COPY . .
RUN cargo build --release

FROM debian:latest
RUN apt-get update && \
       apt-get install -y \
       libpq5 \
       --no-install-recommends
COPY --from=0 /usr/src/myapp/target/release/webservice /usr/local/bin
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/webservice"]
