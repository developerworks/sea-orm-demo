#!/bin/bash

# 数据库配置已经在 .env 中配置, 也可以在命令行通过 <-u, --database-url> 参数覆盖
# --expanded-format: 扩展格式, 可以定制化, 比如: 程序中的字段名称和数据不一致
# --compact-format:  默认格式, 缺少定制化
sea-orm-cli generate entity --lib -o entity/src --with-serde both --tables trauma_patient --expanded-format -v


# 压缩格式的实体文件
sea-orm-cli generate entity -u mysql://root:root@localhost/_wingamingbak --with-serde both --compact-format  --output-dir src/entity_compact -v
# 扩展格式的实体文件
sea-orm-cli generate entity -u mysql://root:root@localhost/_wingamingbak --with-serde both --expanded-format --output-dir src/entity_expanded -v