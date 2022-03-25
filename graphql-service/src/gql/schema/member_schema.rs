use diesel::PgConnection;
use uuid::Uuid;

use error::error::Error;
use yugabyte::context::GraphQLContext;
use yugabyte::engine::member::{
    filter_members_by_name, find_member_by_id, get_all_member_names_by_team_id,
    insert_bulk_members, list_all_members, update_member,
};
use yugabyte::model::dto::PaginationDTO;
use yugabyte::model::member::{Member, Name, NewMember};
use yugabyte::util::utils::current_timestamp;
use juniper::{RootNode, EmptySubscription};


pub struct Query;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Query {
    pub fn list_members(pagination_dto: PaginationDTO, context: &GraphQLContext) -> Result<Vec<Member>, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        list_all_members(&pagination_dto, pg_connection)
    }

    pub fn find_member_by_id(auth_user_id: Uuid, context: &GraphQLContext) -> Result<Member, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        find_member_by_id(&auth_user_id, pg_connection)
    }

    pub fn filter_members_by_the_name(member_name: String, context: &GraphQLContext) -> Result<Vec<Member>, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        filter_members_by_name(&member_name, pg_connection)
    }

    pub fn retrieve_all_member_names_by_team_id(
        team_id: Uuid, context: &GraphQLContext,
    ) -> Result<Vec<Name>, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        get_all_member_names_by_team_id(&team_id, pg_connection)
    }
}


pub struct Mutation;

#[juniper::object(Context = GraphQLContext)]
impl Mutation {
    pub fn create_member(
        context: &GraphQLContext,
        new_member: NewMember,
    ) -> Result<Member, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        new_member.insert_member(pg_connection)
    }

    pub fn create_bulk_members(
        context: &GraphQLContext,
        new_members: Vec<NewMember>,
    ) -> Result<Vec<Member>, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        let mut members = Vec::new();

        // Step 2: Iterate over the New Teams and create the list of teams to be added in a bulk not to load the execution time of the database.
        for new_member in new_members {
            let member = Member {
                id: Uuid::new_v4(),
                team_id: new_member.team_id,
                user_id: new_member.user_id,
                name: new_member.name,
                identity_num: new_member.identity_num,
                role: new_member.role,
                assigned_at: current_timestamp(),
                expired_at: new_member.expired_at,
                modification_date: None,
            };
            members.push(member);
        }

        insert_bulk_members(&members, pg_connection)
    }

    pub fn update_one_member(
        context: &GraphQLContext,
        member: Member,
    ) -> Result<Member, Error> {
        let pg_connection: &PgConnection = &context.pool.get().unwrap();

        update_member(&member, pg_connection)
    }
}


pub type MemberSchema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn member_schema() -> MemberSchema {
    MemberSchema::new(Query, Mutation, EmptySubscription::new())
}