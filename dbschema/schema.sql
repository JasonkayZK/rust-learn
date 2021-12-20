CREATE TABLE IF NOT EXISTS url_maps
(
    key VARCHAR(50) PRIMARY KEY,
    url TEXT NOT NULL
);

INSERT INTO url_maps (key, url)
VALUES ('qq', 'https://qq.com'),
       ('google', 'https://google.com'),
       ('facebook', 'https://facebook.com'),
       ('twitter', 'https://twitter.com'),
       ('bilibili', 'https://bilibili.com');
