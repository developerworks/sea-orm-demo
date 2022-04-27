#!/bin/bash
 
sea-orm-cli generate entity \
    -u mysql://root:root@localhost/_wingamingbak \
    -o src/entity \
    --with-serde both \
    -v