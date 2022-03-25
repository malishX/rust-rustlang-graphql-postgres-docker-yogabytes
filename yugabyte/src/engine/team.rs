use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use error::error::Error;

use crate::model::dto::PaginationDTO;
use crate::model::team::{NewTeam, Team};
use crate::schema::team::dsl::{description, name, team};
use crate::schema::team::dsl::id as team_id;

impl NewTeam {
    pub fn insert_team(&self, connection: &PgConnection) -> Result<Team, Error> {
        let initialized_member = Team {
            id: Uuid::new_v4(),
            name: self.name.clone(), // I cloned the name only not the whole team object because the string is located in the heap memory.
            description: self.description.clone() // I cloned the description only not the whole team object because the string is located in the heap memory.
        };
        diesel::insert_into(team)
            .values(initialized_member)
            .get_result(connection)
            .map_err(|_| Error::DuplicationError)
    }
}

pub fn insert_bulk_team(
    other_teams: &Vec<Team>,
    connection: &PgConnection,
) -> Result<Vec<Team>, Error> {
    diesel::insert_into(team)
        .values(other_teams)
        .get_results::<Team>(connection)
        .map_err(|_| Error::DuplicationError)
}

pub fn list_all_teams(
    pagination_dto: &PaginationDTO,
    connection: &PgConnection,
) -> Result<Vec<Team>, Error> {
    team
        .limit(pagination_dto.page_size as i64)
        .offset(pagination_dto.offset as i64)
        .load::<Team>(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn count_teams(connection: &PgConnection) -> Result<i64, Error> {
    team
        .count()
        .get_result(connection)
        .map_err(|e| Error::DBError(e))
}

pub fn delete_team_by_id(other_team_id: &Uuid, connection: &PgConnection) -> bool {
    match diesel::delete(team.filter(team_id.eq(other_team_id)))
        .execute(connection)
        .map_err(|e| Error::DBError(e)) {
        Ok(0) => false,
        Ok(1) => true,
        _ => false,
    }
}

pub fn delete_all_teams(connection: &PgConnection) -> Result<usize, Error> {
    diesel::delete(team)
        .execute(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn find_team_by_id(
    other_team_id: &Uuid,
    connection: &PgConnection,
) -> Result<Team, Error> {
    team
        .find(other_team_id)
        .get_result::<Team>(connection)
        .map_err(|err| Error::DBError(err))
}


pub fn update_auth_user(
    incoming_team: &Team,
    connection: &PgConnection,
) -> Result<Team, Error> {
    diesel::update(team.find(&incoming_team.id))
        .set((
            name.eq(&incoming_team.name),
            description.eq(&incoming_team.description),
        ))
        .get_result::<Team>(connection)
        .map_err(|e| Error::DBError(e))
}
