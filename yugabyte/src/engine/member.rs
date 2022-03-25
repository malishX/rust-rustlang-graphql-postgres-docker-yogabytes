use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use error::error::Error;

use crate::model::dto::PaginationDTO;
use crate::model::member::{Member, Name, NewMember};
use crate::schema::member::dsl::{expired_at, identity_num, member, modification_date, name, role};
use crate::schema::member::dsl::id as member_id;
use crate::util::utils::current_timestamp;

impl NewMember {
    pub fn insert_member(&self, connection: &PgConnection) -> Result<Member, Error> {
        let initialized_member = Member {
            id: Uuid::new_v4(),
            team_id: self.team_id,
            user_id: self.user_id,
            name: self.name.clone(),
            identity_num: self.identity_num.clone(),
            role: self.role.clone(),
            assigned_at: current_timestamp(),
            expired_at: self.expired_at,
            modification_date: None,
        };
        diesel::insert_into(member)
            .values(initialized_member)
            .get_result(connection)
            .map_err(|_| Error::DuplicationError)
    }
}

pub fn insert_bulk_members(
    other_members: &Vec<Member>,
    connection: &PgConnection,
) -> Result<Vec<Member>, Error> {
    diesel::insert_into(member)
        .values(other_members)
        .get_results::<Member>(connection)
        .map_err(|_| Error::DuplicationError)
}

pub fn list_all_members(
    pagination_dto: &PaginationDTO,
    connection: &PgConnection,
) -> Result<Vec<Member>, Error> {
    member
        .limit(pagination_dto.page_size as i64)
        .offset(pagination_dto.offset as i64)
        .load::<Member>(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn count_members(connection: &PgConnection) -> Result<i64, Error> {
    member
        .count()
        .get_result(connection)
        .map_err(|e| Error::DBError(e))
}

pub fn delete_member_by_id(other_member_id: &Uuid, connection: &PgConnection) -> bool {
    match diesel::delete(member.filter(member_id.eq(other_member_id)))
        .execute(connection)
        .map_err(|e| Error::DBError(e)) {
        Ok(0) => false,
        Ok(1) => true,
        _ => false,
    }
}

// todo: // todo: need to return the list of deleted items to use it in the GraphQL like what I did in the auth_user engine
pub fn delete_all_members(connection: &PgConnection) -> Result<usize, Error> {
    diesel::delete(member)
        .execute(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn find_member_by_id(
    other_member_id: &Uuid,
    connection: &PgConnection,
) -> Result<Member, Error> {
    member
        .find(other_member_id)
        .get_result::<Member>(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn update_member(
    incoming_member: &Member,
    connection: &PgConnection,
) -> Result<Member, Error> {
    diesel::update(member.find(&incoming_member.id))
        .set((
            name.eq(&incoming_member.name),
            identity_num.eq(&incoming_member.identity_num),
            role.eq(&incoming_member.role),
            modification_date.eq(current_timestamp()),
            expired_at.eq(&incoming_member.expired_at),
        ))
        .get_result::<Member>(connection)
        .map_err(|e| Error::DBError(e))
}

pub fn filter_members_by_name(
    other_name: &String,
    connection: &PgConnection,
) -> Result<Vec<Member>, Error> {
    member
        .filter(name.eq(other_name))
        .get_results::<Member>(connection)
        .map_err(|e| Error::DBError(e))
}

pub fn get_all_member_names_by_team_id(
    other_team_id: &Uuid,
    connection: &PgConnection,
) -> Result<Vec<Name>, Error> {
    let query = format!("SELECT name FROM member WHERE team_id::text = '{}'", other_team_id);
    diesel::sql_query(query)
        .load::<Name>(connection)
        .map_err(|e| Error::DBError(e))
}