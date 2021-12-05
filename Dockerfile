####################################################################################################
## Builder
####################################################################################################
ARG BASE_IMAGE=rust:slim-buster

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

RUN apt update && apt install -y build-essential cmake libsasl2-dev zlib1g-dev
RUN update-ca-certificates

# Create appuser
ENV USER=rocky
ENV UID=1000

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /app

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:buster-slim

RUN apt update && apt install -y zlib1g-dev

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/rocky-insight ./

# Use an unprivileged user.
USER rocky:rocky

CMD ["/app/rocky-insight"]