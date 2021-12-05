CREATE TABLE IF NOT EXISTS url_maps
(
    key VARCHAR(50) PRIMARY KEY,
    url TEXT NOT NULL
);

INSERT INTO url_maps (key, url)
VALUES ('qq', 'qq.com'),
       ('google', 'google.com'),
       ('facebook', 'facebook.com'),
       ('twitter', 'twitter.com'),
       ('bilibili', 'bilibili.com');
