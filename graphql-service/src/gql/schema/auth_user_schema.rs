use diesel::pg::PgConnection;
use uuid::Uuid;

use error::error::Error;
use yugabyte::context::GraphQLContext;
use yugabyte::engine::auth_user::{
    delete_all_auth_users, find_auth_user_by_id, insert_bulk_auth_users, list_all_auth_users,
};
use yugabyte::model::auth_user::AuthUser;
use yugabyte::model::dto::PaginationDTO;
use yugabyte::model::user::NewUser;
use juniper::{RootNode, EmptySubscription, GraphQLType, Registry, DefaultScalarValue};
use juniper::meta::MetaType;

pub struct Query;

// The root Query struct depends on GraphQLContext to provide the connection pool
// needed to execute the actual Postgres queries.
#[juniper::graphql_object(Context = GraphQLContext)]
impl Query {
    // I think the reason of error is that I need to change the Return type to FieldResult,
    // and my engine function returns Result<Vec<AuthUser>, Error>
    pub fn list_auth_users(pagination_dto: PaginationDTO, context: &GraphQLContext) -> Result<Vec<AuthUser>, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        list_all_auth_users(&pagination_dto, pg_connection)
    }

    pub fn find_auth_user(auth_user_id: Uuid, context: &GraphQLContext) -> Result<AuthUser, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        find_auth_user_by_id(&auth_user_id, pg_connection)
    }
}


pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Mutation {
    pub fn create_auth_user(
        context: &GraphQLContext,
        new_user: NewUser,
    ) -> Result<AuthUser, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        new_user.add_auth_user(pg_connection)
    }

    pub fn create_bulk_auth_user(
        context: &GraphQLContext,
        new_users: Vec<NewUser>,
    ) -> Result<Vec<AuthUser>, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        let mut auth_users = Vec::new();

        // Step 2: Iterate over the New Teams and create the list of teams to be added in a bulk not to load the execution time of the database.
        for new_user in new_users {
            let auth_user = AuthUser {
                id: Uuid::new_v4(),
                email: new_user.email,
                password: new_user.password,
            };
            auth_users.push(auth_user);
        }

        insert_bulk_auth_users(&auth_users, pg_connection)
    }

    pub fn remove_all_auth_user(
        context: &GraphQLContext,
    ) -> Result<Vec<AuthUser>, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        delete_all_auth_users(pg_connection)
    }
}


pub type AuthUserSchema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn auth_user_schema() -> AuthUserSchema {
    AuthUserSchema::new(Query, Mutation, EmptySubscription::new())
}