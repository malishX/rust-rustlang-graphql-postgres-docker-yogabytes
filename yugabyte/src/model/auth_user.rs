use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::auth_user;

/// I created this model separately for security purposes. and I prefer to put this table
/// in another database to prevent knowing the password if the database has been hacked.
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Validate, Apiv2Schema, GraphQLObject, Clone)]
#[table_name = "auth_user"]
pub struct AuthUser {
    pub id: Uuid,
    #[validate(email(code = "email-format-error"))]
    pub email: String,
    #[validate(length(min = 9, max = 127, code = "password-length-error"))]
    pub password: String,
}