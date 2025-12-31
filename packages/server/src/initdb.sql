-- ACME in ascii as a decimal
PRAGMA application_id=1094929733;


CREATE TABLE file_store
(
    id   integer not null
        constraint file_store_pk
            primary key autoincrement,
    data blob    not null
);

create table main.file_to_tags
(
    id      integer not null
        constraint file_to_tags_pk
            primary key autoincrement,
    file_id integer not null
        constraint file_to_tags_file_store_id_fk
            references main.file_store
            on update cascade on delete cascade,
    tag_id  integer not null
        constraint file_to_tags_tags_id_fk
            references main.tags
            on update cascade on delete cascade,
    data    any
);

create index main.file_to_tags_tag_id_data_index
    on main.file_to_tags (tag_id, data);

CREATE TABLE tags
(
    id   integer not null
        constraint tags_pk
            primary key autoincrement,
    name text    not null
        constraint tags_pk_2
            unique
);

CREATE TABLE thumbnails
(
    file_id   integer not null
        constraint thumbnails_pk
            primary key
        constraint thumbnails_file_store_id_fk
            references file_store (id)
            on update cascade on delete cascade,
    thumbnail blob    not null
);

