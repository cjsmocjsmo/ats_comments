#!/bin/bash

cd /usr/share/ats_comments;
rm -rf uploads;
echo "Removed uploads dir";
cd /usr/share/ats_comments/ats_comments;
rm -rf db;
echo "Removed db dir";

git pull;
RUST_LOG=info cargo run --release