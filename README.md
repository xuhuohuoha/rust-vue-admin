2025.2.27


简要说明：

一）基于 Rust + Vue3 + Naive UI + Pinia + Router

二）想法：简化开发，提供权限动态配置体系，开发过程中提供接口和业务表单页面，通过快速配置构建系统

三）其他：仍需进一步开发更多功能，修复bug；


部署说明：

第一步：恢复db文件夹中的数据库（mysql）

第二步：修改配置文件

1、.env配置文件：包含主机、端口、sea-orm-migration（此处数据库连接仅用于 sea-orm-cli）

2、config/config.toml：仅需修改database的url

第三步：启动 exe

第四步：访问系统：默认 http://127.0.0.1/8080 



