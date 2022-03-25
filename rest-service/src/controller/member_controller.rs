use actix_web::web::Path;
use paperclip::actix::{
    api_v2_operation,
    web::{self, Query},
};
use paperclip::actix::web::Json;
use uuid::Uuid;

use error::error::Errors;
use error::error::StateCode::{DBError, NotFound, PaginationError};
use yugabyte::db_connection::{CoreDBPool, pgdata_to_pgconnection};
use yugabyte::engine::member::{count_members, delete_all_members, delete_member_by_id, filter_members_by_name, find_member_by_id, get_all_member_names_by_team_id, insert_bulk_members, list_all_members};
use yugabyte::engine::user::find_user_by_id;
use yugabyte::model::dto::{MemberEmail, MemberInfo, MemberName, PaginatedResponseDTO, PaginationDTO, SuccessResponse};
use yugabyte::model::member::{Member, Name, NewMember};
use yugabyte::util::utils::current_timestamp;

#[api_v2_operation]
pub async fn find_member_email_api(
    user_id: web::Path<Uuid>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<MemberEmail>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Find the user from the database.
    match find_user_by_id(&user_id, &pg_connection) {
        Ok(found_user) => {
            let member_email = MemberEmail {
                name: found_user.name,
                email: found_user.email,
            };
            // Step 3: Fire the response
            Ok(Json(SuccessResponse {
                message: format!("Successfully find the Member Email."),
                data: member_email,
            }))
        }
        Err(_) => Err(Errors::NotFound(NotFound.into()))
    }
}

#[api_v2_operation]
pub async fn find_member_info_api(
    web::Path((user_id, member_id)): web::Path<(Uuid, Uuid)>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<MemberInfo>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Find the user from the database.
    match find_user_by_id(&user_id, &pg_connection) {
        Ok(found_user) => {
            // Step 3: Find the member from the database.
            match find_member_by_id(&member_id, &pg_connection) {
                Ok(found_member) => {
                    let member_info = MemberInfo {
                        name: found_user.name,
                        email: found_user.email,
                        identity_num: found_member.identity_num,
                        role: found_member.role,
                    };
                    // Step 4: Fire the response
                    Ok(Json(SuccessResponse {
                        message: format!("Successfully find the Member Info."),
                        data: member_info,
                    }))
                }
                Err(_) => Err(Errors::NotFound(NotFound.into()))
            }
        }
        Err(_) => Err(Errors::NotFound(NotFound.into()))
    }
}

#[api_v2_operation]
pub(crate) fn list_members_api(
    Query(pagination_dto): Query<PaginationDTO>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<PaginatedResponseDTO<Member>>>, Errors> {
    // Step 1: Get the connection from pool data.
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Count all members.
    match count_members(&pg_connection) {
        Ok(members_count) => {
            // Step 3: List all paginated members.
            match list_all_members(&pagination_dto, &pg_connection) {
                Ok(paginated_list) => {
                    let response = PaginatedResponseDTO {
                        paginated_list,
                        count: members_count,
                    };

                    // Step 4: Fire the response.
                    Ok(Json(SuccessResponse {
                        message: format!("Successfully retrieved all members."),
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
pub(crate) fn insert_member_api(
    new_member: Json<NewMember>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<Member>>, Errors> {
    // Step 1: Get the connection from pool data.
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Insert the member into the database
    match new_member.insert_member(&pg_connection) {
        // Step 3: Fire the inserted member
        Ok(inserted_member) => Ok(Json(SuccessResponse {
            message: format!("Successfully added the new Member."),
            data: inserted_member,
        })),
        Err(_) => Err(Errors::InternalServerError(DBError.into()))
    }
}

#[api_v2_operation]
pub(crate) fn insert_bulk_members_api(
    new_members: Json<Vec<NewMember>>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<Vec<Member>>>, Errors> {
    // Step 1: Get the connection from pool data.
    let pg_connection = pgdata_to_pgconnection(pool);
    let mut members = Vec::new();

    // Step 2: Iterate over the New Members and create the list of members to be added in a bulk not to load the execution time of the database.
    for new_member in new_members.0 {
        let member = Member {
            id: Uuid::new_v4(),
            team_id: new_member.team_id,
            user_id: new_member.user_id,
            name: new_member.name.clone(),
            identity_num: new_member.identity_num.clone(),
            role: new_member.role.clone(),
            assigned_at: current_timestamp(),
            expired_at: new_member.expired_at,
            modification_date: None,
        };
        members.push(member);
    }

    /*new_members.0.iter().map(|new_member| {
        let member = Member {
            id: Uuid::new_v4(),
            team_id: new_member.team_id,
            user_id: new_member.user_id,
            name: new_member.name.clone(),  // I cloned the name only not the whole new_member object because the string is located in the heap memory.
            identity_num: new_member.identity_num.clone(),
            role: new_member.role.clone(),
            assigned_at: current_timestamp(),
            expired_at: new_member.expired_at,
            modification_date: None,
        };
        members.push(member);
        members.clone()
    });*/

    // Step 4: Insert the bulk of members into the database.
    match insert_bulk_members(&members, &pg_connection) {
        // Step 5: Fire the inserted members
        Ok(inserted_members) => Ok(Json(SuccessResponse {
            message: format!("Successfully added the bulk of Members."),
            data: inserted_members,
        })),
        Err(_) => Err(Errors::InternalServerError(DBError.into()))
    }
}

#[api_v2_operation]
pub async fn remove_member_api(
    member_id: web::Path<Uuid>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<bool>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Delete the member from the database.
    if !delete_member_by_id(&member_id, &pg_connection) {
        Err(Errors::InternalServerError(DBError.into()))
    } else {
        // Step 3: Fire the response.
        Ok(Json(SuccessResponse {
            message: format!("Successfully deleted the member."),
            data: true,
        }))
    }
}

#[api_v2_operation]
pub async fn remove_all_members_api(
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<bool>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Delete all members from the database.
    if delete_all_members(&pg_connection).is_ok() {
        // Step 3: Fire the response.
        Ok(Json(SuccessResponse {
            message: format!("Successfully deleted all members."),
            data: true,
        }))
    } else {
        Err(Errors::InternalServerError(DBError.into()))
    }
}

#[api_v2_operation]
pub async fn filter_members_by_name_api(
    other_name: Json<MemberName>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<Vec<Member>>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Filter members by name.
    match filter_members_by_name(&other_name.name, &pg_connection) {
        // Step 3: Fire the response.
        Ok(filtered_members) => Ok(Json(SuccessResponse {
            message: format!("Successfully retrieved the filtered members."),
            data: filtered_members,
        })),
        Err(_) => Err(Errors::InternalServerError(DBError.into()))
    }
}

#[api_v2_operation]
pub async fn get_all_member_names_related_to_team_api(
    team_id: Path<Uuid>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<SuccessResponse<Vec<Name>>>, Errors> {
    // Step 1: Get the connection from pool data
    let pg_connection = pgdata_to_pgconnection(pool);

    // Step 2: Filter member names related to the required team.
    match get_all_member_names_by_team_id(&team_id, &pg_connection) {
        // Step 3: Fire the response.
        Ok(member_names) => Ok(Json(SuccessResponse {
            message: format!("Successfully retrieved all member names."),
            data: member_names,
        })),
        Err(_) => Err(Errors::InternalServerError(DBError.into()))
    }
}

