// use std::env;
// use std::error::Error;
// use std::io::ErrorKind;
// use diesel::{RunQueryDsl, SelectableHelper};
// use diesel_async::{AsyncConnection, AsyncMysqlConnection};
// use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
// use diesel::ExpressionMethods;
// use diesel::associations::HasTable;
// use crate::db::Repo;
// use crate::models::{UserRegisterSchema, AuthUserPostgres, NewUser, AuthUser};
// use crate::response::GenericResponse;
// use crate::schema::auth_users::dsl::*;
//
// /// this repo compiles : cargo rustc -- --cfg mysql
// pub struct MysqlRepo {
//     conn: AsyncMysqlConnection,
// }
//
// //#[cfg(all(mysql))]
// impl Repo for MysqlRepo {
//
//     async fn init()
//         -> Result<Box<Self>, Box<dyn Error + Send + Sync>>
//     {
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let conn = AsyncMysqlConnection::establish(&database_url).await
//             .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
//
//         Ok( Box::new(Self { conn } ))
//     }
//
//     async fn find_user_by_email(&mut self, _email: &str) -> Option<AuthUser>
//     {
//         let users: Result<Vec<AuthUser>, _> = auth_users
//             .filter(email.eq(_email))
//             .load(&mut self.conn)
//             .await;
//
//         return if users.is_ok() {
//             users.unwrap().get(0).cloned()
//         } else {
//             None
//         }
//     }
//
//     async fn find_user_by_id(&mut self, _id: &str) -> Option<AuthUser>
//     {
//         let users: Result<Vec<AuthUser>, _> = auth_users
//             .filter(id.eq(_id))
//             .load(&mut self.conn)
//             .await;
//
//         return if users.is_ok() {
//             users.unwrap().get(0).cloned()
//         } else {
//             None
//         }
//     }
//
//     async fn register_user_by_email(&mut self, user: UserRegisterSchema)
//                                     -> Result<GenericResponse, Box<dyn Error>>
//     {
//         //use crate::schema::auth_users;
//         return if self.find_user_by_email( &user.email).await.is_none() {
//
//             let new_user = NewUser {
//                 //id: Uuid::new_v4(),
//                 email: &user.email,
//                 name: &user.name,
//                 password: &user.password,
//                 otp_enabled: None,
//                 otp_verified: None,
//                 otp_base32: None,
//                 otp_auth_url: None,
//                 created_at: Some(chrono::offset::Utc::now()),
//                 updated_at: Some(chrono::offset::Utc::now()),
//             };
//
//             diesel::insert_into(auth_users::table)
//                 .values(&new_user)
//                 .returning(AuthUser::as_returning())
//                 .get_result(&mut self.conn)
//                 .await?;
//
//             Ok(GenericResponse::ok(&format!("registered new user with email: {}", &user.email)))
//         } else {
//             // TODO handle error correctly
//             Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)))
//         }
//     }
// }