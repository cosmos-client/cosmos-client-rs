#!/usr/bin/env bash

rm -r ./src/proto
cp -r ~/src/github.com/cosmos/cosmos-sdk/proto ./src/
cp -r ~/src/github.com/cosmos/cosmos-sdk/third_party/proto ./src/

PATH="$HOME/.cargo/bin:$PATH"

proto_dirs=$(find ./src/proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)

for dir in $proto_dirs; do
  protoc \
    -I src/proto \
    --rust_out ./src/generated \
    $(find "${dir}" -maxdepth 1 -name '*.proto')
done

rm -r ./src/proto