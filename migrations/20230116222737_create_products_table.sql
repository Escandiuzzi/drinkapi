-- Add migration script here
CREATE TABLE products(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   name TEXT NOT NULL,
   description TEXT NOT NULL,
   attributes TEXT NOT NULL,
   images_urls TEXT NOT NULL,
   created_at timestamptz NOT NULL,
   updated_at timestamptz NOT NULL
);