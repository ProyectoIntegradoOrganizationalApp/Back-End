use crate::models::models::*;
use diesel::prelude::*;
use crate::schema::*;
use diesel::result::Error;
use rust_api_rest::establish_connection;

pub fn is_admin(idproject: &str, iduser: &str) -> Result<UserProject, GenericError> {
    println!("{}", idproject);
    println!("{}", iduser);
    let connection = &mut establish_connection();
    let user_found: Result<User, Error> = users::table.filter(users::id.eq(&iduser)).first::<User>(connection);
    match user_found {
        Ok(user) => {
            let user_project_found: Result<UserProject, Error> = UserProject::belonging_to(&user)
                                    .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
                                    .filter(projects::idproject.eq(idproject))
                                    .filter(project_user::idrole.eq("1".to_string()))
                                    .select(UserProject::as_select())
                                    .get_result::<UserProject>(connection);
            match user_project_found {
                Ok(user_project) => {
                    Ok(user_project)
                },
                Err(_) => Err(GenericError { error: true, message: "The user is not a member of the project or is not an Administrator".to_owned() })
            }
        },
        Err(_) => {
            Err(GenericError { error: true, message: "The user provided was not found".to_owned() })
        }
    }
}

pub fn get_user_projects(projects: Vec<Project>, connection: &mut PgConnection) -> Vec<UserProjectsDetail>{
    let mut projects_info:Vec<UserProjectsDetail> = Vec::new();
    for project in &projects {
        match get_project_members(&project, connection) {
            Ok(project_members) => {
                let user_projects_info = UserProjectsDetail {
                    id: project.idproject.clone(),
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