use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use diesel::types::VarChar;
use juniper::GraphQLInputObject;
use juniper::GraphQLObject;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::member;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone, GraphQLObject, Apiv2Schema)]
#[table_name = "member"]
pub struct Member {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub identity_num: String,
    pub role: String,
    pub assigned_at: NaiveDateTime,
    pub expired_at: Option<NaiveDateTime>,
    pub modification_date: Option<NaiveDateTime>,
}

#[derive(Default, Debug, Serialize, Deserialize, Apiv2Schema, GraphQLInputObject)]
pub struct NewMember {
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub identity_num: String,
    pub role: String,
    pub expired_at: Option<NaiveDateTime>,
}

#[derive(Debug, QueryableByName, GraphQLObject, Serialize, Deserialize)]
pub struct Name {
    #[sql_type = "VarChar"]
    pub name: String,
}