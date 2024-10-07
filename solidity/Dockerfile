FROM ubuntu:jammy-20240212 as solc-builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    cmake \
    git \
    libboost-all-dev \
    && \
    update-ca-certificates

WORKDIR /app/solidity
RUN git clone https://github.com/klkvr/solidity . && git checkout 88cf606

RUN mkdir build && \
    cd build && \
    cmake .. -DCMAKE_BUILD_TYPE=Release \
             -DCMAKE_CXX_FLAGS="-Wno-error=conversion" && \
    make solc -j$(nproc)

FROM ubuntu:jammy-20240212

COPY --from=solc-builder /app/solidity/build/solc/solc /app/solc
WORKDIR /app/root

ENTRYPOINT ["/app/solc"]