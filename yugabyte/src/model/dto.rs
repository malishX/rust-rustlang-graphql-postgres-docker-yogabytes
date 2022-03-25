use juniper::GraphQLInputObject;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Apiv2Schema, GraphQLInputObject, Debug)]
pub struct PaginationDTO {
    pub page_size: i32,
    pub offset: i32,
}

#[derive(Default, Serialize, Apiv2Schema, Debug)]
pub struct PaginatedResponseDTO<T> {
    pub paginated_list: Vec<T>,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
pub struct SuccessResponse<T> {
    pub message: String,
    pub data: T,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct MemberEmail {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct MemberInfo {
    pub name: String,
    pub email: String,
    pub identity_num: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct MemberName {
    pub name: String,
}

