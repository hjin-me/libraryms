create table change_logs
(
    id          bigserial                not null,
    operator    text                     not null,
    source_id   bigint                   not null,
    source_type text                     not null,
    action      text,
    operate_at  timestamp with time zone not null
);

