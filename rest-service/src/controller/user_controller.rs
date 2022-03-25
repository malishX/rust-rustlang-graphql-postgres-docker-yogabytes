use paperclip::actix::{
    api_v2_operation,
    web::{self, Query},
};
use paperclip::actix::web::Json;

use error::error::Errors;
use error::error::StateCode::{DBError, PaginationError};
use yugabyte::db_connection::{CoreDBPool, pgdata_to_pgconnection};
use yugabyte::engine::user::{count_users, list_all_users};
use yugabyte::model::dto::{PaginatedResponseDTO, PaginationDTO, SuccessResponse};
use yugabyte::model::user::{NewUser, User};

#[api_v2_operation]
pub(crate) fn list_users_api(
    Query(pagination_dto): Query<PaginationDTO>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<PaginatedResponseDTO<User>>>, Errors> {
    // Step 1: Get the connection from pool data.
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Count all users.
    match count_users(&pg_connection) {
        Ok(users_count) => {
            // Step 3: List all paginated users.
            match list_all_users(&pagination_dto, &pg_connection) {
                Ok(paginated_list) => {
                    let response = PaginatedResponseDTO {
                        paginated_list,
                        count: users_count,
                    };

                    // Step 4: Fire the response.
                    Ok(Json(SuccessResponse {
                        message: format!("Successfully retrieved all users."),
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
pub(crate) fn insert_user_api(
    new_user: Json<NewUser>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<User>>, Errors> {
    // Step 1: Get the connection from pool data.
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Insert the User into the database
    match new_user.add_user(&pg_connection) {
        // Step 3: Fire the inserted user
        Ok(inserted_user) => Ok(Json(SuccessResponse {
            message: format!("Successfully added the new User."),
            data: inserted_user,
        })),
        Err(_) => Err(Errors::InternalServerError(DBError.into()))
    }
}