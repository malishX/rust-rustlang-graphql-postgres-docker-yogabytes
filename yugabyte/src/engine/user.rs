use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use error::error::Error;

use crate::model::dto::PaginationDTO;
use crate::model::user::{NewUser, User};
use crate::schema::user::dsl::user;

impl NewUser {
    pub fn add_user(&self, connection: &PgConnection) -> Result<User, Error> {
        let initialized_user = User {
            id: Uuid::new_v4(),
            email: self.email.clone(),
            name: self.name.clone(),
        };
        diesel::insert_into(user)
            .values(&initialized_user)
            .get_result(connection)
            .map_err(|_| Error::DuplicationError)
    }
}

pub fn insert_bulk_users(
    other_users: &Vec<User>,
    connection: &PgConnection,
) -> Result<Vec<User>, Error> {
    diesel::insert_into(user)
        .values(other_users)
        .get_results::<User>(connection)
        .map_err(|_| Error::DuplicationError)
}

pub fn list_all_users(
    pagination_dto: &PaginationDTO,
    connection: &PgConnection,
) -> Result<Vec<User>, Error> {
    user
        .limit(pagination_dto.page_size as i64)
        .offset(pagination_dto.offset as i64)
        .load::<User>(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn count_users(connection: &PgConnection) -> Result<i64, Error> {
    user
        .count()
        .get_result(connection)
        .map_err(|e| Error::DBError(e))
}

pub fn find_user_by_id(
    other_user_id: &Uuid,
    connection: &PgConnection,
) -> Result<User, Error> {
    user
        .find(other_user_id)
        .get_result::<User>(connection)
        .map_err(|err| Error::DBError(err))
}
