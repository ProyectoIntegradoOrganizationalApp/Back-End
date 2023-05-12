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
    }
}

diesel::table! {
    achievement_user (idachievement, iduser) {
        idachievement -> Varchar,
        iduser -> Varchar,
        progress -> Int2,
        completed -> Bool,
    }
}

diesel::table! {
    app (id) {
        id -> Varchar,
        idproject -> Varchar,
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
    docs (idapp, idproject) {
        idapp -> Varchar,
        idproject -> Varchar,
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
    kanban (idapp, idproject) {
        idapp -> Varchar,
        idproject -> Varchar,
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
    project_user (id, iduser) {
        id -> Varchar,
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
    projects (id) {
        id -> Varchar,
        iduser -> Varchar,
        name -> Varchar,
        configuration -> Json,
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
    timeline (idapp, idproject) {
        idapp -> Varchar,
        idproject -> Varchar,
    }
}

diesel::table! {
    user_friend (iduser, idfriend) {
        iduser -> Varchar,
        idfriend -> Varchar,
    }
}

diesel::table! {
    user_invitation (idproject, idguest, iduser) {
        idproject -> Varchar,
        idguest -> Varchar,
        iduser -> Varchar,
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
    }
}

diesel::joinable!(achievement_user -> achievement (idachievement));
diesel::joinable!(columna -> board (idboard));
diesel::joinable!(project_user -> role (idrole));
diesel::joinable!(task -> columna (idcolumn));
diesel::joinable!(user_friend -> users (idfriend));
diesel::joinable!(user_invitation -> users (idguest));

diesel::allow_tables_to_appear_in_same_query!(
    achievement,
    achievement_user,
    app,
    board,
    columna,
    docs,
    goal,
    kanban,
    notification,
    project_user,
    project_user_activity,
    projects,
    recent_change,
    review,
    role,
    task,
    timeline,
    user_friend,
    user_invitation,
    users,
);
