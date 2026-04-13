# Build Stage
FROM ubuntu:24.04 as builder

## Add dependencies
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake curl libcairo2-dev libpango1.0-dev libxml2-dev build-essential libgdk-pixbuf2.0-dev

## Install rust nightly, and cargo-fuzz
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN ${HOME}/.cargo/bin/rustup default nightly
RUN ${HOME}/.cargo/bin/cargo install -f cargo-fuzz

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd fuzz && ${HOME}/.cargo/bin/cargo fuzz build && popd" && \
    mv target/x86_64-unknown-linux-gnu/release/render_document /render_document && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd fuzz && ${HOME}/.cargo/bin/cargo build --release && popd" && \
    mv target/release/render_document /render_document_no_inst && \
    echo done

RUN mkdir /testsuite && find /src -name '*.svg' -exec cp {} /testsuite/ \;

FROM ubuntu:24.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    libcairo2 libcairo-gobject2 libpango-1.0-0 libpangocairo-1.0-0 libgdk-pixbuf-2.0-0 \
    libxml2 libglib2.0-0 libfontconfig1

COPY --from=builder /render_document /render_document_no_inst /
RUN mkdir /testsuite
COPY --from=builder /testsuite/ /testsuite/
