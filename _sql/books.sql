create table books
(
    id           bigserial,
    isbn         text,
    title        text                     not null,
    authors      text[],
    publisher    text,
    publish_date text,
    state_id     bigint                   not null,
    thumbnail    text,
    created_at   timestamp with time zone not null,
    deleted_at   timestamp with time zone
);

alter table books
    owner to postgres;

INSERT INTO public.books (id, isbn, title, authors, publisher, publish_date, state_id, thumbnail, created_at, deleted_at) VALUES (1, '9787121390746', 'Go语言编程之旅：一起用Go做项目', '{陈剑煜,徐新华}', '电子工业出版社', '2020-6', 1, 'https://img1.doubanio.com/view/subject/m/public/s33682507.jpg', '2022-12-06 03:50:51.295882 +00:00', null);
