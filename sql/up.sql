-- USER
create type public.privacy as enum(
    'private', 'public', 'with_users', 'with_group', 'with_topic'
);

CREATE TYPE public.user_status AS ENUM ('offline', 'online', 'inactive', 'deactivated');

create type public.status as enum ('active', 'archived', 'deleted');

CREATE TABLE public.users (
    id       uuid not null primary key,
    email    text,
    username text not null
    password text,
    status   user_status default 'offline'::public.user_status,
    created  timestamptz default timezone('utc'::text, now()) not null
);

CREATE TABLE public.topics (
    id      uuid not null primary key,
    name    text not null,
    created timestamptz default timezone('utc'::text, now()) not null
);

create table public.records (
    id       uuid not null primary key,
    name     text not null,
    status   status not null default 'active'::public.status
    created timestamptz default timezone('utc'::text, now()) not null

);

create table public.facts (
    id       uuid not null primary key,
    name    text not null,
    created timestamptz default timezone('utc'::text, now()) not null

);

create table public.items (
    id       uuid not null primary key,
    name     text not null,
    created timestamptz default timezone('utc'::text, now()) not null

);

create table public.groups (
    id      uuid not null primary key,
    uid     uuid not null references public.users,
    name    text not null,
    created timestamptz default timezone('utc'::text, now()) not null


);

create table public.messages (
    id           uuid not null primary key,
    sender       uuid not null references public.users,
    recipient    uuid not null references public.users,
    content      text,
    created      timestamptz default timezone('utc'::text, now()) not null
);

create table public.topic_post (
    id          uuid not null primary key,
    uid         uuid not null references public.users,
    topic       uuid not null references public.topics,
    content     text,
    created     timestamptz default timezone('utc'::text, now()) not null
);
