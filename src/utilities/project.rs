use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::*;

pub fn get_user_projects(user: &User, connection: &mut PgConnection) -> Result<Vec<UserProjectsDetail>, String> {
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

pub fn get_project_user_activity(user: &User, connection: &mut PgConnection) -> Result<Vec<UserActivityProfile>, String> {
    let activity_found = ProjectUserActivity::belonging_to(&user)
            .select(ProjectUserActivity::as_select())
            .load::<ProjectUserActivity>(connection);
            
    match activity_found {
        Ok(activity) => {
            let mut activity_info:Vec<UserActivityProfile> = Vec::new();
            for i in &activity {
                let user_activity_info = UserActivityProfile {
                    idproject: i.idproject.clone(),
                    date: i.date.clone(),
                    commits: i.commits
                };
                activity_info.push(user_activity_info);
            }
            Ok(activity_info)
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

pub fn is_member_project(project: &Project, user_id: &String, connection: &mut PgConnection) -> Result<User, String> {
    let project_member_found = UserProject::belonging_to(&project)
        .inner_join(users::table.on(project_user::iduser.eq(users::id)))
        .select(User::as_select())
        .filter(project_user::iduser.eq(&user_id))
        .get_result::<User>(connection);
    match project_member_found {
        Ok(member) => Ok(member),
        Err(_) => Err("The user is not a member of the project".to_owned())
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