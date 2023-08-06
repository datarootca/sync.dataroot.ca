CREATE TABLE IF NOT EXISTS category (
    categoryid SERIAL primary key,
    name varchar(63) not null unique,
    slug varchar(63),
    is_active boolean default true,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);