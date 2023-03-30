create table accounts
(
    id           text                     not null
        constraint pk_id
            primary key,
    display_name text                     not null,
    role         text                     not null,
    created_at   timestamp with time zone not null
);

