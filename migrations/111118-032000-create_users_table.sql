create table users
(
	uuid uuid default uuid_generate_v4() not null
		constraint users_pkey
			primary key,
	first_name varchar(20) not null,
	last_name varchar(30) not null,
	date_of_birth timestamp
);

alter table users owner to tweeter;

create unique index users_uuid_uindex
	on users (uuid);