INSERT INTO users (id, email, password_hash, name)
VALUES ('4d2d5cad-618e-46aa-aff7-eb5332857b95', 'me@example.com',
        '$argon2id$v=19$m=4096,t=3,p=1$heModEtYf09qetag2en3dA$SYKz0sn8DtQmBLHJYiYgnQP1C9RSJEGdTFjCituP6dU', 'Me User')
ON CONFLICT (id) DO UPDATE SET
    email = EXCLUDED.email,
    password_hash = EXCLUDED.password_hash,
    name = EXCLUDED.name;
