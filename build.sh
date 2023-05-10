#!/bin/bash

cargo build --release

if [ $? -eq 0 ]; then
    echo "Multigrep binary has been built successfully!"
else
    echo "Mulgrep binary build failed!"
    exit 1
fi

sudo cp target/release/multigrep /usr/local/bin

if [ $? -eq 0 ]; then
    echo "Installation successful!"
else
    echo "Installation failed!"
    exit 1
fi
