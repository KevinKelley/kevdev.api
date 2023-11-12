use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    // embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        //.map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
        .map_err(|e| CustomError{code:500, message:format!("Failed getting db connection: {}", e)})
}



// let engine = "mysql";
// let username = "root";
// let password = "123456";
// let host = "localhost";
// let port = 3306;
// let db = "test";
// let url = format!(
//     "{}://{}:{}@{}:{}/{}",
//     engine, username, password, host, port, db
// );
// let connection = MysqlConnection::establish(&url)
//     .expect(&format!("Failed to connect database:{}-{}", engine, db));
// Database { connection }
