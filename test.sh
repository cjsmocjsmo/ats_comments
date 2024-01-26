#!/bin/bash

# A1='https://atstest.xyz/addcom';
# A2='/Herman Munster';
# A3='/herman@monster.com';
# A4='/this is the first comment lets see if it works';
# A5='/5';
# A6=$A1$A2$A3$A4$A5;
# A7=$(echo $A6 | sed 's/ /%20/g');
# curl $A7;

# B1='https://atstest.xyz/addcom';
# B2='/Casper The Ghost';
# B3='/casper@monster.com';
# B4='/this is another commit from casper';
# B5='/4';
# B6=$B1$B2$B3$B4$B5;
# B7=$(echo $B6 | sed 's/ /%20/g');
# curl $B7; 

# curl https://atstest.xyz/allcom;

C1='https://atstest.xyz/addesti'
C2='/foo bar'
C3='/814 Hull Ave'
C4='/Port Orchard'
C5='/903-465-7811'
C6='/foo@gmail.com'
C7='/A tree fell I need help'
C8='/01-02-2023';
C9=$C1$C2$C3$C4$C5$C6$C7$C8;
C10=$(echo $C9 | sed 's/ /%20/g');
curl $C10;

D2='/charlie smotherman';
D3='/924 hull ave';
D4='/port orchard';
D5='/903-465-7811';
D6='/charlie@gmail.com';
D7='/great job';
D8='/01-02-2023';
D9=$C1$D2$D3$D4$D5$D6$D7$D8;
D10=$(echo $D9 | sed 's/ /%20/g');
curl $D10;

curl https://atstest.xyz/allesti;


