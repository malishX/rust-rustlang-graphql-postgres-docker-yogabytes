table! {
    auth_user (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
    }
}

table! {
    member (id) {
        id -> Uuid,
        team_id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        identity_num -> Varchar,
        role -> Varchar,
        assigned_at -> Timestamp,
        expired_at -> Nullable<Timestamp>,
        modification_date -> Nullable<Timestamp>,
    }
}

table! {
    team (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    user (id) {
        id -> Uuid,
        email -> Varchar,
        name -> Varchar,
    }
}

joinable!(member -> team (team_id));
joinable!(member -> user (user_id));

allow_tables_to_appear_in_same_query!(
    auth_user,
    member,
    team,
    user,
);
