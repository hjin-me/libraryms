create table books
(
    id           bigserial,
    isbn         text,
    title        text                     not null,
    authors      text[],
    publisher    text,
    publish_date text,
    state        text,
    log_id       bigint                   not null,
    thumbnail    text,
    created_at   timestamp with time zone not null,
    deleted_at   timestamp with time zone
);

