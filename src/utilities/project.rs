use crate::models::models::*;
use chrono::Utc;
use diesel::{prelude::*};
use crate::schema::*;

fn pass_admin(project_id: &String, connection: &mut PgConnection) -> Result<(), GenericError> {
    let user: Result<UserProject, _> = project_user::table
        .filter(project_user::idproject.eq(project_id))
        .order(project_user::joined_at.desc())
        .first::<UserProject>(connection);

    match user {
        Ok(mut user) => {
            user.idrole = "1".to_owned();
            let saved = user.save_changes::<UserProject>(connection);
            match saved {
                Ok(_) => Ok(()),
                Err(_) => Err(GenericError { error: true, message: "Error passing admin".to_owned() })
            }
        },
        Err(_) => {
            let deleted_project = diesel::delete(projects::table.filter(projects::idproject.eq(project_id))).execute(connection);
            match deleted_project {
                Ok(_) => Ok(()),
                Err(_) => Err(GenericError { error: true, message: "An error ocurred deleting the project".to_owned() })
            }
        }
    }
}

pub fn check_update_admin_user_left(project_id: &String, connection: &mut PgConnection) -> Result<(), GenericError> {

    let found_admin = project_user::table
        .filter(project_user::idproject.eq(project_id))
        .filter(project_user::idrole.eq("1"))
        .first::<UserProject>(connection);

    match found_admin {
        Ok(_) => Ok(()),
        Err(_) => {
            match pass_admin(project_id, connection) {
                Ok(_) => Ok(()),
                Err(err) => Err(err)
            }
        }
    }
}

fn get_user_projects_details(projects: Vec<Project>, connection: &mut PgConnection) -> Vec<UserProjectsDetail> {
    let mut projects_info: Vec<UserProjectsDetail> = Vec::new();
    for project in &projects {
        match get_project_members(&project, connection) {
            Ok(project_members) => {
                let user_projects_info = UserProjectsDetail {
                    idproject: project.idproject.clone(),
                    name: project.name.clone(),
                    description: project.description.clone(),
                    icon: project.icon.clone(),
                    updated_at: project.updated_at.clone(),
                    members: project_members
                };
                projects_info.push(user_projects_info);
            },
            Err(_) => ()
        };
    }
    projects_info
}

pub fn create_project_user(project_id: &String, user_id: &String, role: &str, connection: &mut PgConnection) -> Result<UserProject, GenericError> {
    let new_project_user = UserProject {
        idproject: project_id.clone(),
        iduser: user_id.clone(),
        idrole: role.to_string(),
        joined_at: Utc::now().naive_utc()
    };
    let created_project_user = diesel::insert_into(project_user::table)
        .values(&new_project_user)
        .get_result::<UserProject>(connection);
    match created_project_user {
        Ok(user_project) => Ok(user_project),
        Err(_) => Err(GenericError { error: false, message: "Error joining to project".to_string() })
    }
}

pub fn get_user_projects(user: &User, request_id: &String, connection: &mut PgConnection) -> Result<Vec<UserProjectsDetail>, String> {
    let projects_found = UserProject::belonging_to(&user)
        .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
        .filter(projects::state.eq(1))
        .select(Project::as_select())
        .load::<Project>(connection);
    match projects_found {
        Ok(mut projects) => {
            let mut common_projects = get_common_private_projects(&user.id, &request_id, connection);
            if common_projects.len() != 0 {
                projects.append(&mut common_projects);
            }
            let projects_info: Vec<UserProjectsDetail> = get_user_projects_details(projects, connection);
            Ok(projects_info)
        },
        Err(_) => Err("Projects not found".to_string())
    }
}

pub fn get_own_projects(user: &User, connection: &mut PgConnection) -> Result<Vec<UserProjectsDetail>, String> {
    let projects_found = UserProject::belonging_to(&user)
        .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
        .select(Project::as_select())
        .load::<Project>(connection);
    match projects_found {
        Ok(projects) => {
            let projects_info: Vec<UserProjectsDetail> = get_user_projects_details(projects, connection);
            Ok(projects_info)
        },
        Err(_) => Err("Error getting your own projects".to_string())
    }
}

pub fn get_project_user(project: &Project, user_id: &String, connection: &mut PgConnection) -> Result<UserProject, GenericError> {
    let guest_in_project_found = UserProject::belonging_to(project)
        .inner_join(users::table.on(project_user::iduser.eq(users::id)))
        .select(UserProject::as_select())
        .filter(project_user::iduser.eq(user_id))
        .get_result::<UserProject>(connection);
    match guest_in_project_found {
        Ok(guest) => Ok(guest),
        Err(_err) => Err(GenericError { error: true, message: "The user is not a member of the project".to_string()})
    }
}

pub fn get_project_members(project: &Project, connection: &mut PgConnection) -> Result<Vec<ProjectMembers>, String> {
    let members_found = UserProject::belonging_to(&project)
        .inner_join(users::table.on(project_user::iduser.eq(users::id)))
        .select((User::as_select(), UserProject::as_select()))
        .load::<(User, UserProject)>(connection);
    let mut project_members: Vec<ProjectMembers> = Vec::new();
    match members_found {
        Ok(members) => {
            for member in &members {
                let role_found = role::table.filter(role::id.eq(&member.1.idrole))
                    .first::<Role>(connection);
                match role_found {
                    Ok(role) => {
                        let project_members_info = ProjectMembers {
                            id: member.0.id.clone(),
                            name: member.0.name.clone(),
                            photo: member.0.photo.clone(),
                            idrole: role.id.clone(),
                            role: role.name.clone()
                        };
                        project_members.push(project_members_info);
                    },
                    Err(_) => ()
                };
            }
            Ok(project_members)
        },
        Err(_) => Err("Members not found".to_string())
    }
}

pub fn get_project(project_id: &String, connection: &mut PgConnection) -> Result<Project, String> {
    let project_found = projects::table
        .select(Project::as_select())
        .filter(projects::idproject.eq(&project_id))
        .get_result::<Project>(connection);
    match project_found {
        Ok(project) => Ok(project),
        Err(_) => Err("Couldn't find a project with that id".to_owned())
    }
}

pub fn create_project_invitation(project_id: &String, guest_id: &String, user_id: &String, invitation: &InvitationMessage, connection: &mut PgConnection) -> Result<UserInvitation, String> {
    let new_invitation = UserInvitation {
        idproject: project_id.clone(),
        idguest: guest_id.clone(),
        iduser: user_id.clone(),
        title: invitation.title.clone(),
        message: invitation.message.clone()
    };
    let created_invitation = diesel::insert_into(user_invitation::table)
        .values(&new_invitation)
        .get_result::<UserInvitation>(connection);
    match created_invitation {
        Ok(user_invitation) => Ok(user_invitation),
        Err(_) => Err("An error ocurred creating the user invitation".to_owned())
    }
}

pub fn get_project_invitation(project_id: &String, guest_id: &String, user_id: &String, connection: &mut PgConnection) -> Result<UserInvitation, GenericError>{
    let invitation_found = user_invitation::table
        .select(UserInvitation::as_select())
        .filter(user_invitation::idproject.eq(&project_id))
        .filter(user_invitation::iduser.eq(&user_id))
        .filter(user_invitation::idguest.eq(&guest_id))
        .get_result::<UserInvitation>(connection);
    match invitation_found {
        Ok(invitation) => Ok(invitation),
        Err(_) => Err(GenericError { error: true, message: "An error ocurred trying to find the invitation".to_owned() })
    }
}

pub fn check_and_update_role(user: &mut UserProject, id_role: &String, connection: &mut PgConnection) -> Result<(), GenericError> {
    let idrole_number = id_role.clone().parse::<i32>();
    match idrole_number {
        Ok(number) => {
            if number > 0 && number <= 3 {
                user.idrole = id_role.clone();
                let updated_user_role = user.save_changes::<UserProject>(connection);
                match updated_user_role {
                    Ok(_) => Ok(()), 
                    Err(_) => Err(GenericError {error: true, message: "An error ocurred updating user role".to_owned()})
                }
            } else {
                Err(GenericError { error: true, message: "Invalid role".to_string() })
            }
        },
        Err(_) => Err(GenericError {error: true, message: "Invalid Role, not a number".to_owned()})
    }
}

pub fn get_common_private_projects(user_id: &String, request_id: &String, connection: &mut PgConnection) -> Vec<Project> {
    let projects_fail:Vec<Project> = Vec::new();
    let user_private_projects = project_user::table
        .inner_join(projects::table.on(projects::idproject.eq(project_user::idproject)))
        .select(projects::idproject)
        .filter(project_user::iduser.eq(&user_id))
        .filter(projects::state.eq(2))
        .load::<String>(connection);
    match user_private_projects {
        Ok(user_projects) => {
            let request_user_projects_found = project_user::table
                .select(project_user::idproject)
                .filter(project_user::iduser.eq(&request_id))
                .load::<String>(connection);
            match request_user_projects_found {
                Ok(request_user_projects) => {
                    let mut common_projects_id:Vec<String> = Vec::new();
                    for project_id in &request_user_projects {
                        if user_projects.contains(project_id) {
                            common_projects_id.push(project_id.clone());
                        }
                    }
                    if common_projects_id.len() != 0 {
                        let common_projects_found = projects::table
                            .filter(projects::idproject.eq_any(common_projects_id))
                            .load::<Project>(connection);
                        match common_projects_found {
                            Ok(common_projects) => common_projects,
                            Err(_) => projects_fail
                        }
                    } else {
                        projects_fail
                    }
                },
                Err(_) => projects_fail
            }
        },
        Err(_) => projects_fail
    }
}