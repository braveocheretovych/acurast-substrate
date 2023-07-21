FROM rust:latest AS builder
RUN apt update && apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
RUN rustup update nightly-2023-03-14 && rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-14

WORKDIR /code
COPY . .

RUN \
if [ "${chain}" = "kusama" ] ; \
then cargo +nightly-2023-03-14 build --no-default-features --features 'proof-of-authority,std' --release ; \
else cargo +nightly-2023-03-14 build --release ; \
fi

# adapted from https://github.com/paritytech/polkadot/blob/master/scripts/ci/dockerfiles/polkadot/polkadot_builder.Dockerfile
FROM docker.io/library/ubuntu:20.04

COPY --from=builder /code/target/release/acurast-node /usr/local/bin/

RUN useradd -m -u 1000 -U -s /bin/sh -d /app app && \
	mkdir /data && \
	chown -R app:app /data && \
# check if executable works in this container
	/usr/local/bin/acurast-node --version

USER app

ENTRYPOINT ["/usr/local/bin/acurast-node"]
CMD [ "help" ]
