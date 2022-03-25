use actix_web::HttpResponse;
use dotenv::dotenv;
use paperclip::actix::web;
use paperclip::actix::web::ServiceConfig;
use tracing_subscriber::EnvFilter;

use crate::controller::auth_user_controller::{
    find_auth_user_by_id_api, insert_auth_user_api, list_auth_users_api,
    remove_all_auth_users_api, remove_auth_user_api,
};
use crate::controller::member_controller::{
    filter_members_by_name_api, find_member_email_api, find_member_info_api,
    get_all_member_names_related_to_team_api, insert_bulk_members_api, insert_member_api,
    list_members_api, remove_all_members_api, remove_member_api,
};
use crate::controller::team_controller::{
    find_team_by_id_api, insert_bulk_teams_api, insert_team_api, list_teams_api,
    remove_all_teams_api, remove_team_api,
};
use crate::controller::user_controller::{insert_user_api, list_users_api};

pub(crate) mod auth_user_controller;
pub(crate) mod member_controller;
pub(crate) mod team_controller;
pub(crate) mod user_controller;

pub fn routes(config: &mut ServiceConfig) {
    config
        .route("/health", web::get().to(|| HttpResponse::Ok().json("Hello World!!")))
        .service(
            web::scope("/auth_user")
                .route("/list_paginated", web::get().to(list_auth_users_api))
                .route("/insert", web::post().to(insert_auth_user_api))
                .route("/remove/{auth_user_id}", web::delete().to(remove_auth_user_api))
                .route("/remove_all", web::delete().to(remove_all_auth_users_api))
                .route("/find/{auth_user_id}", web::get().to(find_auth_user_by_id_api)),
        )
        .service(
            web::scope("/member")
                .route("/find_email", web::get().to(find_member_email_api))
                .route("/find_info", web::get().to(find_member_info_api))
                .route("/list_paginated", web::get().to(list_members_api))
                .route("/insert", web::post().to(insert_member_api))
                .route("/insert_bulk", web::post().to(insert_bulk_members_api))
                .route("/remove/{member_id}", web::delete().to(remove_member_api))
                .route("/remove_all", web::delete().to(remove_all_members_api))
                .route("/filter_by_name", web::get().to(filter_members_by_name_api))
                .route("/member_names_by_team_id/{team_id}", web::get().to(get_all_member_names_related_to_team_api)),
        )
        .service(
            web::scope("/team")
                .route("/list", web::get().to(list_teams_api))
                .route("/insert", web::post().to(insert_team_api))
                .route("/insert_bulk", web::post().to(insert_bulk_teams_api))
                .route("/remove/{team_id}", web::delete().to(remove_team_api))
                .route("/remove_all", web::delete().to(remove_all_teams_api))
                .route("/find/{team_id}", web::get().to(find_team_by_id_api)),
        )
        .service(
            web::scope("user")
                .route("/list", web::get().to(list_users_api))
                .route("/insert", web::post().to(insert_user_api))
        );
}

// Initiate the tracing subscriber for RUST_LOG
pub fn start_tracing() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}