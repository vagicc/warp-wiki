-- admins表
CREATE TABLE admins (
    id serial primary key,
    username CHARACTER VARYING(16) NOT NULL,
    "password" CHARACTER VARYING(40) NOT NULL,
    salt CHARACTER(10) NOT NULL,
    "email" CHARACTER VARYING(100) DEFAULT NULL,
    "mobile" CHARACTER(11) DEFAULT NULL,
    "role" smallint DEFAULT NULL,
    "status" bigint DEFAULT 0,
    "create_time" TIMESTAMP WITHOUT time ZONE DEFAULT clock_timestamp(),
    --不带时区
    -- create_time TIMESTAMP(6) WITH TIME ZONE DEFAULT clock_timestamp(), --带时区
    "last_login" TIMESTAMP WITHOUT time ZONE DEFAULT NULL
);

CREATE INDEX idx_admins_username ON admins USING btree(username);

CREATE INDEX idx_admins_email ON admins USING btree(email);

CREATE INDEX idx_admins_role ON admins USING btree(role);

COMMENT ON TABLE admins IS '后台管理角色组';

COMMENT ON COLUMN admins.id IS '自增主键ID';

COMMENT ON COLUMN admins.username IS '登录名';

COMMENT ON COLUMN admins.password IS '登录密码';

COMMENT ON COLUMN admins.salt IS '混淆码';

COMMENT ON COLUMN admins.email IS '邮箱';

COMMENT ON COLUMN admins.mobile IS '电话';

COMMENT ON COLUMN admins.role IS '角色组ID';

COMMENT ON COLUMN admins.status IS '是否冻结：0=正常，1=永久冻结，冻结时间';

COMMENT ON COLUMN admins.create_time IS '创建时间(不带时区)';

COMMENT ON COLUMN admins.last_login IS '最后登录时间(不带时区)';

INSERT INTO
    admins (
        "id",
        "username",
        "password",
        "salt",
        "email",
        "mobile",
        "role",
        "status",
        "create_time",
        "last_login"
    )
VALUES
    (
        1,
        'luck',
        'c2a3b691ee173bbaee19a5d6aae8c995507fa706',
        '25ee364a54',
        'luck@fmail.pro',
        '13428122341',
        1,
        0,
        '2008-08-18 18:58:13',
        '2018-08-18 18:58:18'
    );