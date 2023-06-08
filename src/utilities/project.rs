use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::*;

pub fn get_user_projects(user: &User, request_id: &String, connection: &mut PgConnection) -> Result<Vec<UserProjectsDetail>, String> {
    let projects_found:Result<_, _>;
    projects_found = UserProject::belonging_to(&user)
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
            let mut projects_info:Vec<UserProjectsDetail> = Vec::new();
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
            Ok(projects_info)
        },
        Err(err) => Err(err.to_string())
    }
}

pub fn get_own_projects(user: &User, connection: &mut PgConnection) -> Result<Vec<UserProjectsDetail>, String> {
    let projects_found = UserProject::belonging_to(&user)
        .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
        .select(Project::as_select())
        .load::<Project>(connection);
    match projects_found {
        Ok(projects) => {
            let mut projects_info:Vec<UserProjectsDetail> = Vec::new();
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
            Ok(projects_info)
        },
        Err(err) => Err(err.to_string())
    }
}

pub fn get_project_members(project: &Project, connection: &mut PgConnection) -> Result<Vec<ProjectMembers>, String> {
    let members_found = UserProject::belonging_to(&project)
        .inner_join(users::table.on(project_user::iduser.eq(users::id)))
        .select((User::as_select(), UserProject::as_select()))
        .load::<(User, UserProject)>(connection);
    let mut project_members:Vec<ProjectMembers> = Vec::new();
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
        Err(err) => Err(err.to_string())
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

pub fn is_admin_project(project: &Project, user_id: &String, connection: &mut PgConnection) -> Result<User, String> {
    let user_in_project_found = UserProject::belonging_to(&project)
        .inner_join(users::table.on(project_user::iduser.eq(users::id)))
        .select(User::as_select())
        .filter(project_user::iduser.eq(&user_id))
        .filter(project_user::idrole.eq("1"))
        .get_result::<User>(connection);
    match user_in_project_found {
        Ok(user) => Ok(user),
        Err(err) => Err(err.to_string())
    }
}

pub fn is_member_project(project: &Project, user_id: &String, connection: &mut PgConnection) -> bool {
    let project_member_found = UserProject::belonging_to(&project)
        .inner_join(users::table.on(project_user::iduser.eq(users::id)))
        .select(User::as_select())
        .filter(project_user::iduser.eq(&user_id))
        .get_result::<User>(connection);
    match project_member_found {
        Ok(_member) => true,
        Err(_) => false
    }
}

pub fn create_project_invitation(project_id: &String, guest_id: &String, user_id: &String, invitation: &InvitationMessage,  connection: &mut PgConnection) -> Result<UserInvitation, String> {
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
        Err(err) => Err(err.to_string())
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