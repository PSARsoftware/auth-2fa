use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::mongo::MongoRepo;
//use crate::db::sql::mysql::MysqlRepo;
use crate::db::sql::postgres::PostgresRepo;
//use crate::db::sql::sqlite::SqliteRepo;
use crate::models::{AuthUser, UserRegisterSchema};
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
}

impl GenericRepo {
    
    /// Initialize inner repo
    /// [arg] max_connections is actual only for sql repos, for mongo pass any u32
    async fn init(self, uri: &str)
        -> Result<(), Box<dyn Error + Send + Sync>>
    {
        match self {
            GenericRepo::Mongo { mut repo } => {
                let mongo_repo = MongoRepo::init().await?;
                repo = Arc::new(Mutex::new(*mongo_repo));
                Ok(())
            }
            GenericRepo::Postgres { mut repo } => {
                let postgres_repo = PostgresRepo::init().await?;
                repo = Arc::new(Mutex::new(*postgres_repo));
                Ok(())
            }
        }
    }

    async fn find_user_by_email(&self, email: &str)
        -> Option<AuthUser>
    {
        match self {
            GenericRepo::Mongo { repo } => {
                let mut repo = repo.lock().await;
                repo.find_user_by_email(email).await
            }
            GenericRepo::Postgres { repo } => {
                let mut repo = repo.lock().await;
                repo.find_user_by_email(email).await
            }
        }
    }

    async fn register_user_by_email(&self, user: UserRegisterSchema)
        -> Result<GenericResponse, Box<dyn Error>>
    {
        match self {
            GenericRepo::Mongo { repo } => {
                let mut  repo = repo.lock().await;
                repo.register_user_by_email(user).await
            }
            GenericRepo::Postgres { repo } => {
                let mut repo = repo.lock().await;
                repo.register_user_by_email(user).await
            }
        }
    }
}

/// Specific repos should implement this trait
pub trait Repo {
    async fn init() -> Result<Box<Self>, Box<dyn Error + Send + Sync>>;

    async fn find_user_by_email(&mut self, email: &str) -> Option<AuthUser>;

    async fn find_user_by_id(&mut self, id: &str) -> Option<AuthUser>;

    async fn register_user_by_email(&mut self, user: UserRegisterSchema) -> Result<GenericResponse, Box<dyn Error>>;
}