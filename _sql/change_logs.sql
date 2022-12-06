create table change_logs
(
    id          bigint default nextval('change_log_id_seq'::regclass) not null,
    operator    text                                                  not null,
    source_id   bigint                                                not null,
    source_type text                                                  not null,
    state       text                                                  not null,
    action      text,
    operate_at  timestamp with time zone                              not null
);

alter table change_logs
    owner to postgres;

INSERT INTO public.change_logs (id, operator, source_id, source_type, state, action, operate_at) VALUES (1, 'songsong', 1, 'book', '已入库', '新书第一次入库', '2022-12-06 03:50:51.295882 +00:00');
