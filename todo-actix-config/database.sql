drop table if exists todo_items;
drop table if exists todo_lists;

create table todo_lists (
    id serial primary key,
    title varchar(150)
);

create table todo_items (
    id serial primary key,
    title varchar(150) not null,
    checked boolean not null default false,
    list_id integer not null,
    foreign key (list_id) references todo_lists(id)
);

insert into todo_lists (title) values ('List 1'), ('List 2');
insert into todo_items (title, list_id) 
    values ('Connect to database', 1), ('Do queries', 1);