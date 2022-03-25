use paperclip::actix::{
    api_v2_operation,
    web::{self, Query},
};
use paperclip::actix::web::Json;
use uuid::Uuid;

use error::error::Errors;
use error::error::StateCode::{DBError, NotFound};
use error::error::StateCode::PaginationError;
use yugabyte::db_connection::{CoreDBPool, pgdata_to_pgconnection};
use yugabyte::engine::auth_user::{count_auth_users, delete_all_auth_users, delete_auth_user_by_id, find_auth_user_by_id, insert_bulk_auth_users, list_all_auth_users};
use yugabyte::engine::member::delete_all_members;
use yugabyte::model::auth_user::AuthUser;
use yugabyte::model::dto::{PaginatedResponseDTO, PaginationDTO, SuccessResponse};
use yugabyte::model::user::NewUser;

#[api_v2_operation]
pub(crate) fn list_auth_users_api(
    Query(pagination_dto): Query<PaginationDTO>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<PaginatedResponseDTO<AuthUser>>>, Errors> {
    // Step 1: Get the connection from pool data.
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Count all auth_users.
    match count_auth_users(&pg_connection) {
        Ok(auth_users_count) => {
            // Step 3: List all paginated auth_users.
            match list_all_auth_users(&pagination_dto, &pg_connection) {
                Ok(paginated_list) => {
                    let response = PaginatedResponseDTO {
                        paginated_list,
                        count: auth_users_count,
                    };

                    // Step 4: Fire the response.
                    Ok(Json(SuccessResponse {
                        message: format!("Successfully retrieved all auth users."),
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
pub(crate) fn insert_auth_user_api(
    new_user: Json<NewUser>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<AuthUser>>, Errors> {
    // Step 1: Get the connection from pool data.
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Insert the Auth_User into the database
    match new_user.add_auth_user(&pg_connection) {
        // Step 3: Fire the inserted auth_user
        Ok(inserted_auth_user) => Ok(Json(SuccessResponse {
            message: format!("Successfully added the new Auth User."),
            data: inserted_auth_user,
        })),
        Err(_) => Err(Errors::InternalServerError(DBError.into()))
    }
}

#[api_v2_operation]
pub async fn remove_auth_user_api(
    auth_user_id: web::Path<Uuid>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<bool>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Delete the auth_user from the database.
    if !delete_auth_user_by_id(&auth_user_id, &pg_connection) {
        Err(Errors::InternalServerError(DBError.into()))
    } else {
        // Step 3: Fire the response.
        Ok(Json(SuccessResponse {
            message: format!("Successfully deleted the auth_user."),
            data: true,
        }))
    }
}

#[api_v2_operation]
pub async fn remove_all_auth_users_api(
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<bool>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Delete all auth_users from the database.
    match delete_all_auth_users(&pg_connection) {
        Ok(deleted_auth_users) => {
            // Step 3: Delete all members users, and teams from the database.
            if delete_all_members(&pg_connection).is_ok() {
                // Step 4: Fire the response.
                Ok(Json(SuccessResponse {
                    message: format!("Successfully deleted all auth_users."),
                    data: true,
                }))
            } else {
                // Step 3: In case an error happened while deleting members users, and teams, I will insert the deleted auth users again.
                match insert_bulk_auth_users(&deleted_auth_users, &pg_connection) {
                    Ok(_) => Err(Errors::InternalServerError(DBError.into())),
                    Err(_) => Err(Errors::InternalServerError(DBError.into()))
                }
            }
        }
        Err(_) => Err(Errors::InternalServerError(DBError.into()))
    }
}

#[api_v2_operation]
pub async fn find_auth_user_by_id_api(
    auth_user_id: web::Path<Uuid>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<AuthUser>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Find the auth_user from the database.
    match find_auth_user_by_id(&auth_user_id, &pg_connection) {
        Ok(found_auth_user) => {
            // Step 3: Fire the response
            Ok(Json(SuccessResponse {
                message: format!("Successfully found the Auth User."),
                data: found_auth_user,
            }))
        }
        Err(_) => Err(Errors::NotFound(NotFound.into()))
    }
}

