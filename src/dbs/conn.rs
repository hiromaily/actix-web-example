use sea_orm::{Database, DbErr};

// refer to: https://www.sea-ql.org/sea-orm-tutorial/ch01-01-project-setup.html
pub async fn get_conn(
    user: &str,
    password: &str,
    host: &str,
    db_name: &str,
) -> Result<sea_orm::DatabaseConnection, DbErr> {
    let db_url = format!("postgresql://{user}:{password}@{host}/{db_name}");
    let conn: sea_orm::DatabaseConnection = Database::connect(db_url).await?;
    // let db = &match conn.get_database_backend() {
    //     DbBackend::Postgres => {
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("DROP DATABASE IF EXISTS \"{}\";", *db_name),
    //         ))
    //         .await?;
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("CREATE DATABASE \"{}\";", *db_name),
    //         ))
    //         .await?;

    //         Database::connect(format!("{}/{}", db_url, *db_name)).await?
    //     }
    // };

    Ok(conn)
}
