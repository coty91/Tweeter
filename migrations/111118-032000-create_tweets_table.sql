create table tweets
(
	uuid uuid default uuid_generate_v4() not null
		constraint tweets_pkey
			primary key,
	tweet varchar(280) not null,
	user_uuid uuid not null
		constraint tweets_users_uuid_fk
			references users
);

alter table tweets owner to tweeter;

create unique index tweets_uuid_uindex
	on tweets (uuid);

create index tweets_tweet_index
	on tweets (tweet);

