#!/bin/bash

curl https://atstest.xyz/addcom/Herman%20Munster/herman@monster.com/this%20is%20the%20first%20comment%20lets%20see%20if%20it%20works;

curl https://atstest.xyz/addcom/casper/casper@monster.com/this%20is%20another%20commit%20from%20casper;

curl https://atstest.xyz/allcom;

curl https://atstest.xyz/addesti/foo%20bar/814%20Hull%20Ave/Port@20Orchard/903-465-7811/foo@gmail.com/01-02-2023/01-22-2023;

curl https://atstest.xyz/allesti;

# curl https://atstest.xyz/addcom/Herman Munster/herman@monster.com/this is the first comment lets see if it works;


# SENDMAIL="/usr/share/sendmail/sendmail/sendmail"
# $SENDMAIL \
#     -etype "esti" \
#     -name "Herman Munster" \
#     -address "555 Mockingbird Lane" \
#     -city "Mockingbird Heights" \
#     -phone "555-555-5555" \
#     -email "herman@monster.com" \
#     -comment "This is the first comment lets see if it works" \
#     -intake "01-02-2023";
    