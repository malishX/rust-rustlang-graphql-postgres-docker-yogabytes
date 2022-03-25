use crate::db_connection::PgPool;

pub struct GraphQLContext {
    pub pool: PgPool,
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL objects.
impl juniper::Context for GraphQLContext {}
