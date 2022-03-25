use paperclip::actix::{
    api_v2_operation,
    web::{self, Query},
};
use paperclip::actix::web::Json;
use uuid::Uuid;

use error::error::Errors;
use error::error::StateCode::{DBError, NotFound, PaginationError};
use yugabyte::db_connection::{CoreDBPool, pgdata_to_pgconnection};
use yugabyte::engine::team::{count_teams, delete_all_teams, delete_team_by_id, find_team_by_id, insert_bulk_team, list_all_teams};
use yugabyte::model::dto::{PaginatedResponseDTO, PaginationDTO, SuccessResponse};
use yugabyte::model::team::{NewTeam, Team};

#[api_v2_operation]
pub(crate) fn list_teams_api(
    Query(pagination_dto): Query<PaginationDTO>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<PaginatedResponseDTO<Team>>>, Errors> {
    // Step 1: Get the connection from pool data.
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Count all teams.
    match count_teams(&pg_connection) {
        Ok(teams_count) => {
            // Step 3: List all paginated teams.
            match list_all_teams(&pagination_dto, &pg_connection) {
                Ok(paginated_list) => {
                    let response = PaginatedResponseDTO {
                        paginated_list,
                        count: teams_count,
                    };
                    // Step 4: Fire the response.
                    Ok(Json(SuccessResponse {
                        message: format!("Successfully retrieved all teams."),
                        data: response,
                    }))
                }
                Err(_) => {
                    Err(Errors::BadRequest(PaginationError.into()))
                }
            }
        }
        Err(_) => {
            Err(Errors::InternalServerError(DBError.into()))
        }
    }
}

#[api_v2_operation]
pub(crate) fn insert_team_api(
    new_team: Json<NewTeam>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<Team>>, Errors> {
    // Step 1: Get the connection from pool data.
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Insert the team into the database
    match new_team.insert_team(&pg_connection) {
        // Step 3: Fire the inserted team
        Ok(inserted_team) => Ok(Json(SuccessResponse {
            message: format!("Successfully added the new Team."),
            data: inserted_team,
        })),
        Err(_) => Err(Errors::InternalServerError(DBError.into()))
    }
}

#[api_v2_operation]
pub(crate) fn insert_bulk_teams_api(
    new_teams: Json<Vec<NewTeam>>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<Vec<Team>>>, Errors> {
    // Step 1: Get the connection from pool data.
    let pg_connection = pgdata_to_pgconnection(pool);
    let mut teams = Vec::new();

    // Step 2: Iterate over the New Teams and create the list of teams to be added in a bulk not to load the execution time of the database.
    for new_team in new_teams.0 {
        let team = Team {
            id: Uuid::new_v4(),
            name: new_team.name,
            description: new_team.description,
        };
        teams.push(team);
    }

    // Its performance is not the best, that's why I commented it.
    /*new_teams.0.iter().map(|new_team| {
        let team = Team {
            id: Uuid::new_v4(),
            name: new_team.name.clone(),  // I cloned the name only not the whole new_team object because the string is located in the heap memory.
            description: new_team.description.clone(), // I cloned the description only not the whole new_team object because the string is located in the heap memory.
        };
        teams.push(team);
        teams.clone()
    });*/

    // Step 4: Insert the bulk of teams into the database.
    match insert_bulk_team(&teams, &pg_connection) {
        // Step 5: Fire the inserted teams.
        Ok(inserted_teams) => Ok(Json(SuccessResponse {
            message: format!("Successfully added the bulk of Teams."),
            data: inserted_teams,
        })),
        Err(_) => Err(Errors::InternalServerError(DBError.into()))
    }
}

#[api_v2_operation]
pub async fn remove_team_api(
    team_id: web::Path<Uuid>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<bool>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Delete the team from the database.
    if !delete_team_by_id(&team_id, &pg_connection) {
        Err(Errors::InternalServerError(DBError.into()))
    } else {
        // Step 3: Fire the response.
        Ok(Json(SuccessResponse {
            message: format!("Successfully deleted the team."),
            data: true,
        }))
    }
}

#[api_v2_operation]
pub async fn remove_all_teams_api(
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<bool>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Delete all teams from the database.
    if delete_all_teams(&pg_connection).is_ok() {
        // Step 3: Fire the response.
        Ok(Json(SuccessResponse {
            message: format!("Successfully deleted all teams."),
            data: true,
        }))
    } else {
        Err(Errors::InternalServerError(DBError.into()))
    }
}

#[api_v2_operation]
pub async fn find_team_by_id_api(
    team_id: web::Path<Uuid>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<Team>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Find the team from the database.
    match find_team_by_id(&team_id, &pg_connection) {
        Ok(found_team) => {
            // Step 3: Fire the response
            Ok(Json(SuccessResponse {
                message: format!("Successfully found the Team."),
                data: found_team,
            }))
        }
        Err(_) => Err(Errors::NotFound(NotFound.into()))
    }
}
