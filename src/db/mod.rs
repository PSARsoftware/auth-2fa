use std::error::Error;
use std::sync::Arc;
use sqlx::{MySql, Postgres, Sqlite};
use tokio::sync::Mutex;
use crate::db::mongo::MongoRepo;
use crate::db::sql::mysql::MysqlRepo;
use crate::db::sql::postgres::PostgresRepo;
use crate::db::sql::sqlite::SqliteRepo;
use crate::models::{User, UserRegisterSchema};
use crate::response::GenericResponse;

//#[cfg(all(mongo))]
pub mod mongo;

//#[cfg(any(posgtres,sqlite,mysql))]
pub mod sql;

pub enum GenericRepo {
    Mongo {
        repo: Arc<Mutex<MongoRepo>>,
    },
    Postgres {
        //repo: Arc<Mutex<PostgresRepo<Postgres>>>,
        repo: Arc<Mutex<PostgresRepo>>,
    },
    Mysql {
        // repo: Arc<Mutex<MysqlRepo<MySql>>>,
        repo: Arc<Mutex<MysqlRepo>>,
    },
    Sqlite {
        // repo: Arc<Mutex<SqliteRepo<Sqlite>>>,
        repo: Arc<Mutex<SqliteRepo>>,
    },
}

impl GenericRepo {
    
    /// Initialize inner repo
    /// [arg] max_connections is actual only for sql repos, for mongo pass any u32
    // async fn init(&self, max_connections: u32, uri: &str)
    //     -> Result<Box<Self>, sqlx::Error>
    // {
    //     let repo = self.repo.lock().await;
    //     repo.init(max_connections, uri).await
    // }

    async fn find_user_by_custom_field(&self, field_name: &str, field: &str)
        -> Option<User>
    {
        match self {
            GenericRepo::Mongo { repo } => {
                let repo = repo.lock().await;
                repo.find_user_by_custom_field(field_name, field).await
            }
            GenericRepo::Postgres { repo } => {
                let repo = repo.lock().await;
                repo.find_user_by_custom_field(field_name, field).await
            }
            GenericRepo::Mysql { repo } => {
                let repo = repo.lock().await;
                repo.find_user_by_custom_field(field_name, field).await
            }
            GenericRepo::Sqlite { repo } => {
                let repo = repo.lock().await;
                repo.find_user_by_custom_field(field_name, field).await
            }
        }
    }

    async fn register_user_by_email(&self, user: UserRegisterSchema)
        -> Result<GenericResponse, Box<dyn Error>>
    {
        match self {
            GenericRepo::Mongo { repo } => {
                let repo = repo.lock().await;
                repo.register_user_by_email(user).await
            }
            GenericRepo::Postgres { repo } => {
                let repo = repo.lock().await;
                repo.register_user_by_email(user).await
            }
            GenericRepo::Mysql { repo } => {
                let repo = repo.lock().await;
                repo.register_user_by_email(user).await
            }
            GenericRepo::Sqlite { repo } => {
                let repo = repo.lock().await;
                repo.register_user_by_email(user).await
            }
        }
    }
}

/// Specific repos should implement this trait
pub trait Repo<DB: sqlx::database::Database> {
    async fn init(max_connections: u32, uri: &str) -> Result<Box<Self>, Box<dyn Error + Send + Sync>>;

    async fn find_user_by_custom_field(&self, field_name: &str, field: &str) -> Option<User>;

    async fn register_user_by_email(&self, user: UserRegisterSchema) -> Result<GenericResponse, Box<dyn Error>>;
}