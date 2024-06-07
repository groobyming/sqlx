IF DB_ID('bk-sqlx') IS NULL
    BEGIN
        CREATE DATABASE bk-sqlx;
    END;
GO

USE bk-sqlx;
GO

IF OBJECT_ID('tweet') IS NULL
    BEGIN
        CREATE TABLE tweet
        (
            id       BIGINT          NOT NULL PRIMARY KEY,
            text     NVARCHAR(4000)  NOT NULL,
            is_sent  TINYINT         NOT NULL DEFAULT 1,
            owner_id BIGINT
        );
    END;
GO
