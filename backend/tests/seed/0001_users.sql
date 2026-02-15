INSERT INTO users (id, email, password_hash, name)
VALUES ('96622729-10aa-49e6-b601-0c31ab59f2d4', 'test@example.com',
        '$argon2id$v=19$m=4096,t=3,p=1$heModEtYf09qetag2en3dA$SYKz0sn8DtQmBLHJYiYgnQP1C9RSJEGdTFjCituP6dU', 'Test User')
ON CONFLICT (id) DO UPDATE SET
    email = EXCLUDED.email,
    password_hash = EXCLUDED.password_hash,
    name = EXCLUDED.name;
