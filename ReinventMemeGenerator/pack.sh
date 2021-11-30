#!/bin/bash

echo "building lambda for musl"
cross build --bin lambda --release --target x86_64-unknown-linux-musl && echo "successfully built the lambda"
echo "zipping up lambda"
cp target/x86_64-unknown-linux-musl/release/lambda ./bootstrap && zip lambda.zip bootstrap && rm bootstrap
echo "lambda is packaged and ready for upload"
