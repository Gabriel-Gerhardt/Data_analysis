CREATE TABLE users (
    id SERIAL PRIMARY KEY,         -- Usando SERIAL para o auto incremento
    email VARCHAR UNIQUE NOT NULL, -- Email único e não nulo
    password VARCHAR NOT NULL      -- Senha não nula
);
