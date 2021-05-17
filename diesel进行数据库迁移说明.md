               diesel数据库迁移使用说明
diesel是Rust的ORM(对象关系映射器)和查询构建器
diesel为PostgreSQL、Mysql及SQLite提供了开箱即用的支持
diesel-cli命令行工具（创建、迁移）：
安装diesel-cli工具：cargo install diesel_cli --no-default-features --features postgres

在cargo项目根目录下添加.env文件,加下如下条进行postgres数据库连接配置：
DATABASE_URL=postgres://postgres:llxxs@127.0.0.1:5432/linksnap

在Cargo.toml中添加依赖项：
diesel = { version="1.4.6",features=["extras","postgres","r2d2"] }
dotenv = "0.15.0"

运行"diesel setup"命令生成"migrations"目录与"diesel.toml"文件并且会创建数据库：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel setup
Creating migrations directory at: /luck/Language/Rust/warp-wiki/migrations
Creating database: warpwiki
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$

创建admins表迁移，运行创建表迁移命令（diesel migration generate 表名）：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel migration generate admins
Creating migrations/2021-05-13-071702_admins/up.sql
Creating migrations/2021-05-13-071702_admins/down.sql
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ 
命令运行后会生成两个空的迁移文件up.sql和down.sql,
迁移文件只是普通的SQL,接着在up.sql上面添加CREATE TABLE,同时在down.sql添加相应的DROP TABLE

执行表迁移命令（diesel migration run）：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel migration run
Running migration 2021-05-13-071702_admins
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$
命令执行完后，会在数据库中生成表，同时在项目中生成src/schema.rs文件。

-----------------------------------------------------------------------------------------------
同时会在src/schema.rs中添加相应的(table!宏)表结构.
手动在src/models.rs中添加模型：

学艺购技术架构：
前端：html+css3+js(bootstrap+JQuery)
后端：linux+nginx+rust(warp框架)