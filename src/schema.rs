// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "valid_roles"))]
    pub struct ValidRoles;
}

diesel::table! {
    achievement (id) {
        id -> Varchar,
        title -> Varchar,
        description -> Varchar,
        icon -> Varchar,
        category -> Varchar,
        states -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    achievement_user (idachievement, iduser) {
        idachievement -> Varchar,
        iduser -> Varchar,
        progress -> Int4,
        percentage -> Numeric,
        current_state -> Int4,
        completed -> Bool,
    }
}

diesel::table! {
    app (id) {
        id -> Varchar,
        idproject -> Varchar,
        name -> Varchar,
        description -> Varchar,
        photo -> Varchar,
    }
}

diesel::table! {
    board (id) {
        id -> Varchar,
        idapp -> Varchar,
        title -> Varchar,
    }
}

diesel::table! {
    columna (id) {
        id -> Varchar,
        idboard -> Varchar,
        title -> Varchar,
    }
}

diesel::table! {
    docs_app (idapp, idproject) {
        idapp -> Varchar,
        idproject -> Varchar,
        app_type -> Varchar,
    }
}

diesel::table! {
    goal (id) {
        id -> Varchar,
        idproject -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        completed -> Int2,
    }
}

diesel::table! {
    notification (id) {
        id -> Varchar,
        iduser -> Varchar,
        title -> Varchar,
        content -> Varchar,
        state -> Bool,
    }
}

diesel::table! {
    project_user (idproject, iduser) {
        idproject -> Varchar,
        iduser -> Varchar,
        idrole -> Varchar,
    }
}

diesel::table! {
    project_user_activity (iduser, idproject, date) {
        iduser -> Varchar,
        idproject -> Varchar,
        date -> Varchar,
        commits -> Int2,
    }
}

diesel::table! {
    projects (idproject) {
        idproject -> Varchar,
        iduser -> Varchar,
        name -> Varchar,
        description -> Varchar,
        icon -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}

diesel::table! {
    recent_change (date, idproject) {
        date -> Date,
        idproject -> Varchar,
        backup -> Json,
    }
}

diesel::table! {
    review (id) {
        id -> Varchar,
        iduser -> Varchar,
        title -> Varchar,
        content -> Varchar,
        rating -> Nullable<Numeric>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ValidRoles;

    role (id) {
        id -> Varchar,
        name -> Nullable<ValidRoles>,
    }
}

diesel::table! {
    task (id) {
        id -> Varchar,
        idcolumn -> Varchar,
        title -> Varchar,
        description -> Nullable<Varchar>,
        github -> Nullable<Varchar>,
    }
}

diesel::table! {
    task_app (idapp, idproject) {
        idapp -> Varchar,
        idproject -> Varchar,
        app_type -> Varchar,
    }
}

diesel::table! {
    user_friend (iduser, idfriend) {
        iduser -> Varchar,
        idfriend -> Varchar,
    }
}

diesel::table! {
    user_friend_invitation (idguest, iduser) {
        idguest -> Varchar,
        iduser -> Varchar,
        title -> Varchar,
        message -> Varchar,
    }
}

diesel::table! {
    user_invitation (idproject, idguest, iduser) {
        idproject -> Varchar,
        idguest -> Varchar,
        iduser -> Varchar,
        title -> Varchar,
        message -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        password -> Varchar,
        name -> Varchar,
        lastname -> Varchar,
        phone -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        level -> Int2,
        photo -> Varchar,
    }
}

diesel::joinable!(achievement_user -> achievement (idachievement));
diesel::joinable!(columna -> board (idboard));
diesel::joinable!(project_user -> role (idrole));
diesel::joinable!(task -> columna (idcolumn));
diesel::joinable!(user_friend -> users (idfriend));

diesel::allow_tables_to_appear_in_same_query!(
    achievement,
    achievement_user,
    app,
    board,
    columna,
    docs_app,
    goal,
    notification,
    project_user,
    project_user_activity,
    projects,
    recent_change,
    review,
    role,
    task,
    task_app,
    user_friend,
    user_friend_invitation,
    user_invitation,
    users,
);
