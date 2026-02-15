INSERT INTO users (id, email, password_hash, name)
VALUES ('40e84bb5-b54a-4657-b4bf-dc2c970d1ed6', 'existing@example.com',
        '$argon2id$v=19$m=4096,t=3,p=1$heModEtYf09qetag2en3dA$SYKz0sn8DtQmBLHJYiYgnQP1C9RSJEGdTFjCituP6dU', 'Existing User')
ON CONFLICT (id) DO UPDATE SET
    email = EXCLUDED.email,
    password_hash = EXCLUDED.password_hash,
    name = EXCLUDED.name;
