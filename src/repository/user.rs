use actix::prelude::*;
use diesel::prelude::*;
use crate::common::*;
use crate::repository::Repository;
use crate::model::user::User;
use jsonwebtoken::TokenData;
use crate::repository::schema::*;

#[derive(Debug)]
pub struct Authenticate {
    pub token: String,
}

#[derive(Debug)]
pub struct Authentication {
    pub user: User,
    pub token: String,
}

impl Message for Authenticate {
    type Result = AppResult<Authentication>;
}

impl Handler<Authenticate> for Repository {
    type Result = AppResult<Authentication>;

    fn handle(&mut self, msg: Authenticate, _: &mut Self::Context) -> Self::Result {
        Err(AppError::InternalServerError)
//        use crate::repository::schema::users::dsl::*;
//        let conn = &self.get_conn()?;
//        match users.find(msg.claims_id).first(conn) {
//            Ok(user) => Ok(Authentication {
//                user,
//                token: msg.token,
//            }),
//            Err(e) => Err(e.into()),
//        }
    }
}


#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl Message for RegisterUser {
    type Result = AppResult<User>;
}

impl Handler<RegisterUser> for Repository {
   type Result = AppResult<User>;
   fn handle(&mut self, msg: RegisterUser, ctx: &mut Self::Context) -> Self::Result {
       use crate::repository::schema::users::dsl::*;
       let conn = &self.0.get()?;
       return diesel::insert_into(users)
           .values(msg)
           .get_result(conn)
           .map_err(|err| AppError::InternalServerError);
   }
}