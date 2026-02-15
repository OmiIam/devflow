use sqlx::PgPool;

pub async fn reset_database(pool: &PgPool) {
    sqlx::query("TRUNCATE users CASCADE")
        .execute(pool)
        .await
        .expect("truncate users");
}

pub async fn seed_database(pool: &PgPool) {
    reset_database(pool).await;
    let seed_files = [
        include_str!("../seed/0001_users.sql"),
        include_str!("../seed/0002_users.sql"),
        include_str!("../seed/0003_users.sql"),
    ];

    for file in seed_files {
        sqlx::query(file).execute(pool).await.expect("seed data");
    }
}
