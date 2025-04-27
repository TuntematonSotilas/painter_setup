FROM ghcr.io/ekshore/cargo-leptos-runner-stable:v1.86.0 AS builder

WORKDIR /build
COPY . .

RUN cargo leptos build --release -vv

FROM ubuntu:plucky AS runner

WORKDIR /app

COPY --from=builder /build/target/release/painter_setup /app/painter_setup
COPY --from=builder /build/target/site /app/site

RUN useradd -ms /bin/bash app
USER app

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

CMD [ "/app/painter_setup" ]