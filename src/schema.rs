// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "valid_roles"))]
    pub struct ValidRoles;
}

diesel::table! {
    project_user (id, iduser) {
        id -> Varchar,
        iduser -> Varchar,
        idrole -> Varchar,
    }
}

diesel::table! {
    projects (id) {
        id -> Varchar,
        iduser -> Varchar,
        name -> Varchar,
        configuration -> Json,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ValidRoles;

    role (id) {
        id -> Varchar,
        role -> Nullable<ValidRoles>,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(project_user -> role (idrole));

diesel::allow_tables_to_appear_in_same_query!(
    project_user,
    projects,
    role,
    users,
);
