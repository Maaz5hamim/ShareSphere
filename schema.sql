CREATE DATABASE IF NOT EXISTS sharesphere;
USE sharesphere;

CREATE TABLE IF NOT EXISTS user (
    name     VARCHAR(100)  NOT NULL,
    email    VARCHAR(100)  NOT NULL,
    password VARCHAR(100)  NOT NULL,
    PRIMARY KEY (email)
);

CREATE TABLE IF NOT EXISTS file (
    file_id     INT           NOT NULL AUTO_INCREMENT,
    name        VARCHAR(255)  NOT NULL,
    path        VARCHAR(500)  NOT NULL,
    owner_id    VARCHAR(100)  NOT NULL,
    size        INT UNSIGNED  NOT NULL,
    upload_date TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (file_id),
    FOREIGN KEY (owner_id) REFERENCES user(email) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shared_with (
    file_id INT          NOT NULL,
    email   VARCHAR(100) NOT NULL,
    PRIMARY KEY (file_id, email),
    FOREIGN KEY (file_id) REFERENCES file(file_id) ON DELETE CASCADE,
    FOREIGN KEY (email)   REFERENCES user(email)   ON DELETE CASCADE
);
