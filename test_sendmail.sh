#!/bin/bash

SENDMAIL="/usr/share/sendmail/sendmail/sendmail"
$SENDMAIL \
    -etype "esti" \
    -name "Herman Munster" \
    -address "555 Mockingbird Lane" \
    -city "Mockingbird Heights" \
    -phone "555-555-5555" \
    -email "herman@monster.com" \
    -comment "This is the first comment lets see if it works" \
    -intake "01-02-2023";
    