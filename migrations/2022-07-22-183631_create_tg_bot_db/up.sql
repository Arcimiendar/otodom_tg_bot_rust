-- Your SQL goes here
create table appartment
(
    id          INTEGER not null primary key,
    price       INTEGER,
    czynsz      INTEGER,
    name        VARCHAR,
    rooms       INTEGER,
    scrapped_at DATETIME default CURRENT_TIMESTAMP
);

create table user
(
    id INTEGER not null primary key
);
