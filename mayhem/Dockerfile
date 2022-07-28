# Build Stage
FROM ubuntu:22.04 as builder

## Add dependencies
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake curl libpango1.0-dev libxml2-dev build-essential libgdk-pixbuf2.0-dev

## Install rust nightly, and cargo-fuzz
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN ${HOME}/.cargo/bin/rustup default nightly
RUN ${HOME}/.cargo/bin/cargo install -f cargo-fuzz

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd fuzz && ${HOME}/.cargo/bin/cargo -Z sparse-registry fuzz build && popd" && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/svg_loader /svg_loader && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd fuzz && ${HOME}/.cargo/bin/cargo -Z sparse-registry build --release && popd" && \
    mv fuzz/target/release/svg_loader /svg_loader_no_inst && \
    echo done

RUN mkdir /testsuite && find /src -name '*.svg' -exec cp {} /testsuite/ \;

FROM ubuntu:22.04

COPY --from=builder /svg_loader /svg_loader_no_inst /
RUN mkdir /testsuite
COPY --from=builder /testsuite/ /testsuite/
# copy in shared libraries needed for fuzzing
COPY --from=builder \
            /lib/x86_64-linux-gnu/libcairo-gobject.so.2 \
            /lib/x86_64-linux-gnu/libgdk_pixbuf-2.0.so.0 \
            /lib/x86_64-linux-gnu/libgio-2.0.so.0 \
            /lib/x86_64-linux-gnu/libxml2.so.2 \
            /lib/x86_64-linux-gnu/libpangocairo-1.0.so.0 \
            /lib/x86_64-linux-gnu/libcairo.so.2 \
            /lib/x86_64-linux-gnu/libpango-1.0.so.0 \
            /lib/x86_64-linux-gnu/libgobject-2.0.so.0 \
            /lib/x86_64-linux-gnu/libglib-2.0.so.0 \
            /lib/x86_64-linux-gnu/libgmodule-2.0.so.0 \
            /lib/x86_64-linux-gnu/libpng16.so.16 \
            /lib/x86_64-linux-gnu/libjpeg.so.8 \
            /lib/x86_64-linux-gnu/libz.so.1 \
            /lib/x86_64-linux-gnu/libmount.so.1 \
            /lib/x86_64-linux-gnu/libselinux.so.1 \
            /lib/x86_64-linux-gnu/libicuuc.so.70 \
            /lib/x86_64-linux-gnu/liblzma.so.5 \
            /lib/x86_64-linux-gnu/libpangoft2-1.0.so.0 \
            /lib/x86_64-linux-gnu/libharfbuzz.so.0 \
            /lib/x86_64-linux-gnu/libfontconfig.so.1 \
            /lib/x86_64-linux-gnu/libpixman-1.so.0 \
            /lib/x86_64-linux-gnu/libfreetype.so.6 \
            /lib/x86_64-linux-gnu/libxcb-shm.so.0 \
            /lib/x86_64-linux-gnu/libxcb.so.1 \
            /lib/x86_64-linux-gnu/libxcb-render.so.0 \
            /lib/x86_64-linux-gnu/libXrender.so.1 \
            /lib/x86_64-linux-gnu/libX11.so.6 \
            /lib/x86_64-linux-gnu/libXext.so.6 \
            /lib/x86_64-linux-gnu/libfribidi.so.0 \
            /lib/x86_64-linux-gnu/libthai.so.0 \
            /lib/x86_64-linux-gnu/libffi.so.8 \
            /lib/x86_64-linux-gnu/libpcre.so.3 \
            /lib/x86_64-linux-gnu/libblkid.so.1 \
            /lib/x86_64-linux-gnu/libpcre2-8.so.0 \
            /lib/x86_64-linux-gnu/libicudata.so.70 \
            /lib/x86_64-linux-gnu/libgraphite2.so.3 \
            /lib/x86_64-linux-gnu/libexpat.so.1 \
            /lib/x86_64-linux-gnu/libuuid.so.1 \
            /lib/x86_64-linux-gnu/libbrotlidec.so.1 \
            /lib/x86_64-linux-gnu/libXau.so.6 \
            /lib/x86_64-linux-gnu/libXdmcp.so.6 \
            /lib/x86_64-linux-gnu/libdatrie.so.1 \
            /lib/x86_64-linux-gnu/libbrotlicommon.so.1 \
            /lib/x86_64-linux-gnu/libbsd.so.0 \
            /lib/x86_64-linux-gnu/libmd.so.0 \
            /lib/x86_64-linux-gnu/
