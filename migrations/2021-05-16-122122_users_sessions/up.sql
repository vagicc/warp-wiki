-- 创建用户表与sessions表
CREATE TABLE users(
    "id" SERIAL PRIMARY KEY,
    "email" CHARACTER VARYING(100) DEFAULT NULL,
    "mobile" CHARACTER(11) DEFAULT NULL,
    "username" CHARACTER VARYING(50) NOT NULL,
    "realname" VARCHAR NOT NULL,
    "password" CHARACTER VARYING(40) NOT NULL,
    "salt" CHARACTER(10) DEFAULT NULL,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
    "last_login" TIMESTAMP WITHOUT time ZONE
);
CREATE INDEX idx_users_username ON users USING btree(username);
COMMENT ON TABLE users IS '客户（会员）信息表';
COMMENT ON COLUMN users.email IS '邮箱';
COMMENT ON COLUMN users.mobile IS '手机号码';
COMMENT ON COLUMN users.username IS '会员名';
COMMENT ON COLUMN users.realname IS '真实姓名';

CREATE TABLE sessions(
    "id" SERIAL PRIMARY KEY,
    "cookie" VARCHAR NOT NULL,
    "user_id" INTEGER NOT NULL REFERENCES users(id)
);

CREATE UNIQUE INDEX idx_sessions_cookie ON users (username);