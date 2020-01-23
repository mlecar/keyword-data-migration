use diesel::mysql::MysqlConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_host = env::var("DATABASE_HOST");
    let database_name = env::var("DATABASE_NAME");
    let database_user = env::var("DATABASE_USER");
    let database_password = env::var("DATABASE_PASSWORD");

    let database_connection_string = format!(
        "mysql://{}:{}@{}:3306/{}",
        database_user.unwrap(),
        database_password.unwrap(),
        database_host.unwrap(),
        database_name.unwrap()
    );

    MysqlConnection::establish(&database_connection_string).expect(&format!(
        "Error connecting to {}",
        &database_connection_string
    ))
}
