#!/bin/bash

if [ ! -d "/usr/share/sendmail" ]; then
    echo "Install sendmail";
    exit 1;
fi

if [ -d "/usr/share/sendmail/sendmail" ]; then
    cd /usr/share/sendmail/sendmail;
    git pull;
    go build;
    cd "/usr/share/ats_comments/ats_comments";
fi

if [ ! -d "/usr/share/sendmail/sendmail" ]; then
    cd "/usr/share/sendmail";
    git clone "https://github.com/cjsmocjsmo/sendmail.git";
    go build;
    cd "/usr/share/ats_comments/ats_comments";
fi

if [ -d "/usr/share/ats_comments/uploads" ]; then
    rm -rf "/usr/share/ats_comments/uploads";
fi 
echo "Removed uploads dir";

if [ -d "/usr/share/ats_comments/ats_comments/db"]; then
    rm -rf "/usr/share/ats_comments/ats_comments/db";
fi 
echo "Removed db dir";

git pull;

RUST_LOG=info cargo run --release;
# cargo build --release;

# ATSCOMMENTS = "/usr/share/ats_comments/ats_comments/target/release/ats_comments";

# if [ ! -f /usr/bin/ats_comments ]; then
#     sudo cp -pvr $ATSCOMMENTS /usr/bin/;
#     sudo chown root:root /usr/bin/ats_comments;
#     sudo chmod +x /usr/bin/ats_comments;
# fi

# if [ -f /usr/bin/ats_comments ]; then
#     sudo rm /usr/bin/ats_comments;
#     sudo cp -pvr $ATSCOMMENTS /usr/bin/;
#     sudo chown root:root /usr/bin/ats_comments;
#     sudo chmod +x /usr/bin/ats_comments;
# fi

# SYSD="/etc/systemd/system/";
# SERVFILE="/usr/share/ats_comments/ats_comments/ats_comments.service";
# if [ ! -f $SYSD"/ats_comments.service" ]; then
#     sudo cp -pvr $SERVFILE $SYSD;
# fi 

# sudo systemctl start ats_comments.service;
# sudo systemctl enable ats_comments.service
