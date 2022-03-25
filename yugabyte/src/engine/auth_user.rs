use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::PgConnection;
use diesel::QueryDsl;
use uuid::Uuid;

use error::error::Error;

use crate::model::auth_user::AuthUser;
use crate::model::dto::PaginationDTO;
use crate::model::user::NewUser;
use crate::schema::auth_user::dsl::auth_user;
use crate::schema::auth_user::dsl::id as auth_user_id;

impl NewUser {
    pub fn add_auth_user(&self, connection: &PgConnection) -> Result<AuthUser, Error> {
        // todo: don't forget to hash the password
        let initialized_auth_user = AuthUser {
            id: Uuid::new_v4(),
            email: self.email.clone(),
            password: self.password.clone(),
        };
        diesel::insert_into(auth_user)
            .values(&initialized_auth_user)
            .get_result(connection)
            .map_err(|_| Error::DuplicationError)
    }
}

pub fn insert_bulk_auth_users(
    other_auth_users: &Vec<AuthUser>,
    connection: &PgConnection,
) -> Result<Vec<AuthUser>, Error> {
    diesel::insert_into(auth_user)
        .values(other_auth_users)
        .get_results::<AuthUser>(connection)
        .map_err(|_| Error::DuplicationError)
}

pub fn list_all_auth_users(
    pagination_dto: &PaginationDTO,
    connection: &PgConnection,
) -> Result<Vec<AuthUser>, Error> {
    auth_user
        .limit(pagination_dto.page_size as i64)
        .offset(pagination_dto.offset as i64)
        .load::<AuthUser>(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn count_auth_users(connection: &PgConnection) -> Result<i64, Error> {
    auth_user
        .count()
        .get_result(connection)
        .map_err(|e| Error::DBError(e))
}

// todo: need to return the deleted AuthUser to use it in the GraphQL
pub fn delete_auth_user_by_id(other_auth_user_id: &Uuid, connection: &PgConnection) -> bool {
    match diesel::delete(
        auth_user.filter(auth_user_id.eq(other_auth_user_id))
    )
        .execute(connection)
        .map_err(|e| Error::DBError(e)) {
        // I can return the deleted object but I decided to return bool to show you that I can manage the code.
        Ok(0) => false,
        Ok(1) => true,
        _ => false,
    }
}

pub fn delete_all_auth_users(connection: &PgConnection) -> Result<Vec<AuthUser>, Error> {
    diesel::delete(auth_user)
        .get_results::<AuthUser>(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn find_auth_user_by_id(
    other_auth_user_id: &Uuid,
    connection: &PgConnection,
) -> Result<AuthUser, Error> {
    auth_user
        .find(other_auth_user_id)
        .get_result::<AuthUser>(connection)
        .map_err(|err| Error::DBError(err))
}
