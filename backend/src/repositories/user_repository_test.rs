use sqlx::{Pgpool, Row};
use uuid::Uuid;

#[tokio::test]

async fn insert_and_fetch_user()-> Result < (), sqlx::Error > {

    let database_url;

    let Pool;

    let user_id;

    let email;

    let password_hash;

sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        email,
        password_hash
    )
    .execute(&pool)
    .await?;

    
    let row = sqlx::query!("SELECT id, email, password_hash FROM users WHERE email = $1", email)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.id, user_id);
    assert_eq!(row.email, email);
    assert_eq!(row.password_hash, password_hash);

    // Test passed
    Ok(())


}