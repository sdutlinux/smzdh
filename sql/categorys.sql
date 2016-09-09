DROP TABLE categorys CASCADE;
CREATE TABLE categorys (
id SERIAL PRIMARY KEY,
name VARCHAR(32) NOT NULL,
description VARCHAR(64) NOT NULL,
flags bigint NOT NULL DEFAULT 0,
created TIMESTAMP with time zone DEFAULT now()
);

DROP TABLE post_category CASCADE;
CREATE TABLE post_category (
id SERIAL PRIMARY KEY,
post_id int NOT NULL DEFAULT 0,
category_id int NOT NULL DEFAULT 0,
created TIMESTAMP with time zone DEFAULT now()
);
