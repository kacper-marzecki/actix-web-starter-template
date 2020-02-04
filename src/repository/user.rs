use actix::prelude::*;
use diesel::prelude::*;
use crate::common::*;
use crate::repository::Repository;
use crate::model::user::User;
use jsonwebtoken::TokenData;
use crate::service::authentication::Claims;


#[derive(Debug)]
pub struct Authenticate {
    pub claims_id: TokenData<Claims>,
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
    type Result = AppResult<Auth>;

    fn handle(&mut self, msg: Authenticate, _: &mut Self::Context) -> Self::Result {
        use crate::repository::schema::users::dsl::*;
        let conn = &self.get_conn();


        match users.find(msg.claims_id).first(conn) {
            Ok(user) => Ok(Authentication {
                user,
                token: msg.token,
            }),
            Err(e) => Err(e.into()),
        }
    }
}
