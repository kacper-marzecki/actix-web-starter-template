use crate::common::*;
use crate::model::user::User;
use crate::repository::schema::*;
use crate::repository::ConnectionMgr;
use crate::repository::Repository;
use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::PooledConnection;
use diesel::sql_types::Serial;
use jsonwebtoken::TokenData;
use uuid::Uuid;

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
        let conn = self.get_conn()?;
        if let Ok(true) = user_already_exists(&conn, &msg) {
            Err(AppError::UnprocessableEntity(json!(format!(
                "User with such username or email already exists"
            ))))
        } else {
            diesel::insert_into(users)
                .values(msg)
                .get_result(&conn)
                .map_err(|err| {
                    println!("{:?}", err);
                    AppError::InternalServerError
                })
        }
    }
}

fn user_already_exists(
    conn: &PooledConnection<ConnectionMgr>,
    msg: &RegisterUser,
) -> Result<bool, diesel::result::Error> {
    use crate::repository::schema::users::dsl::*;
    let maybe_users = users
        .select(username)
        .filter(username.eq(&msg.username))
        .or_filter(email.eq(&msg.email))
        .limit(1)
        .load::<String>(conn);
    match maybe_users {
        Ok(res) => Ok(res.is_empty()),
        Err(err) => Err(err),
    }
}
