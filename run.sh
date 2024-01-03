!#/bin/bash

cd /usr/share/ats_comments;
rm -rf uploads;
cd /usr/share/ats_comments/ats_comments;
rm -rf db;

git pull;
RUST_LOG=info cargo run --release