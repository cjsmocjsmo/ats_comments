#!/bin/bash

sudo apt -y update;
sudo apt -y dist-upgrade;
sudo apt -y autoremove;
sudo apt -y autoclean;
sudo apt -y install build-essential sqlite3 libsqlite3-dev openssl libssl-dev pkg-config git curl;
sudo snap install --classic cerbot;
sudo ln -s /snap/bin/certbot /usr/bin/certbot;
sudo certbot certonly --standalone;
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh;