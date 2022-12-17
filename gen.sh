#!/bin/bash
 
sea-orm-cli generate entity -o src/entity --with-serde both --tables trauma_patient -v


# 压缩格式的实体文件
sea-orm-cli generate entity -u mysql://root:root@localhost/_wingamingbak --with-serde both --compact-format  --output-dir src/entity_compact -v
# 扩展格式的实体文件
sea-orm-cli generate entity -u mysql://root:root@localhost/_wingamingbak --with-serde both --expanded-format --output-dir src/entity_expanded -v