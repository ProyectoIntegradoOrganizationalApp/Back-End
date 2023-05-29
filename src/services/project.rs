extern crate redis;

use crate::models::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use rust_api_rest::establish_connection;
use crate::utilities::achievements::*;
use crate::utilities::*;

use chrono::Utc;

pub fn create_project(project_info: &ProjectInputCreate, token_iduser: String) -> Result<Project, GenericError> {
    let connection = &mut establish_connection();
    let project_id = uuid::Uuid::new_v4().to_string();
    let now: String = (Utc::now()).to_string();
    let new_project = Project {
        idproject: project_id.clone(),
        iduser: token_iduser.clone(),
        name: project_info.name.clone(),
        description: project_info.description.clone(),
        icon: project_info.icon.clone(),
        created_at: now.clone(),
        updated_at: now.clone()
    };
    let created_project = diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result::<Project>(connection);

    match created_project {
        Ok(project) => {
            let new_project_user = UserProject {
                idproject: project_id.clone(),
                iduser: token_iduser.clone(),
                idrole: "1".to_string()
            };
            let created_project_user = diesel::insert_into(project_user::table)
                .values(&new_project_user)
                .get_result::<UserProject>(connection);
            match created_project_user {
                Ok(_user_project) => {
                    let achievement_updated = check_update_user_achievement(&project.iduser, "2");
                    match achievement_updated {
                        Ok(_) => Ok(project),
                        Err(err) => Err(err)
                    }
                },
                Err(err) => Err(GenericError { error: false, message: err.to_string() })
            }
        },
        Err(err) => Err(GenericError { error: false, message: err.to_string() })
    }
}

pub fn update_project(project_info: &ProjectInputCreate, user_id: &String, project_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let user_found: Result<User, Error> = users::table.filter(users::id.eq(&user_id)).first::<User>(connection);
    match user_found {
        Ok(user) => {
            let project_found: Result<Project, Error> = UserProject::belonging_to(&user)
                            .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
                            .filter(projects::idproject.eq(project_id))
                            .filter(project_user::idrole.eq("1".to_string()))
                            .select(Project::as_select())
                            .get_result::<Project>(connection);
            
            match project_found {
                Ok(mut project) => {
                    project.name = project_info.name.clone();
                    project.description = project_info.description.clone();
                    project.icon = project_info.icon.clone();
                    project.updated_at = (Utc::now()).to_string();

                    let updated_project = project.save_changes::<Project>(connection);
                    match updated_project {
                        Ok(_user) => Ok(GenericError {error: false, message: "Updated project successfully".to_string()}), 
                        Err(err) => Err(GenericError {error: true, message: err.to_string()})
                    }
                },
                Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to make it".to_string()})
            }
        }, 
        Err(err) => Err(GenericError {error: true, message: err.to_string()})
    }
}

pub fn delete_project(user_id: &String, project_id: &String) -> Result<GenericError, GenericError> {
    let connection = &mut establish_connection();
    let user_found = users::table.filter(users::id.eq(&user_id)).first::<User>(connection);
    match user_found {
        Ok(user) => {
            let project_found: Result<Project, Error> = UserProject::belonging_to(&user)
                            .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
                            .filter(projects::idproject.eq(project_id))
                            .filter(project_user::idrole.eq("1".to_string()))
                            .select(Project::as_select())
                            .get_result::<Project>(connection);
            
            match project_found {
                Ok(project) => {
                    let deleted = diesel::delete(projects::table.filter(projects::idproject.eq(project.idproject))).execute(connection);
                    match deleted {
                        Ok(_) => Ok(GenericError { error: false, message: "Project deleted successfully".to_string() }),
                        Err(err) => Err(GenericError { error: true, message: err.to_string() })
                    }
                },
                Err(_err) => Err(GenericError {error: true, message: "You are not a member or you don't have privileges to make it".to_string()})
            }
        }, 
        Err(err) => Err(GenericError {error: true, message: err.to_string()})
    }
}

pub fn get_project(project_id: &String, token_data: &TokenValidation) -> Result<ProjectDetail, GenericError> {
    let connection = &mut establish_connection();
    let project_found = projects::table.filter(projects::idproject.eq(&project_id)).first::<Project>(connection);
    match project_found {
        Ok(project) => {
            let members_found = UserProject::belonging_to(&project)
                .inner_join(users::table.on(project_user::iduser.eq(users::id)))
                .select((User::as_select(), UserProject::as_select()))
                .load::<(User, UserProject)>(connection);
            let mut project_members:Vec<ProjectMembers> = Vec::new();
            match members_found {
                Ok(members) => {
                    let mut user_member = false;
                    let mut owner = false;
                    for member in &members {
                        if member.0.id == token_data.token_iduser {
                            user_member = true;
                        }
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

                    if user_member {
                        let project_user_token = UserProject::belonging_to(&project)
                            .filter(project_user::iduser.eq(&token_data.token_iduser))
                            .select(UserProject::as_select())
                            .get_result::<UserProject>(connection);
                        match project_user_token {
                            Ok(project_user) => {
                                if project_user.idrole == "1" {
                                    owner = true;
                                }
                            },
                            Err(_) => ()
                        };
                    }

                    let project_detailed = ProjectDetail {
                        idproject: project.idproject.clone(),
                        iduser: project.iduser.clone(),
                        name: project.name.clone(),
                        description: project.description.clone(),
                        icon: project.icon.clone(),
                        created_at: project.created_at.clone(),
                        updated_at: project.updated_at.clone(),
                        members: project_members,
                        owner
                    };

                    Ok(project_detailed)
                },
                Err(err) => Err(GenericError { error: true, message: err.to_string() })
            }
        },
        Err(err) => Err(GenericError { error: true, message: err.to_string() })
    }
}

pub fn get_user_projects(user_id: &String) -> Result<UserProjects, GenericError> {
    let connection = &mut establish_connection();
    let user_found = users::table.filter(users::id.eq(&user_id)).first::<User>(connection);
    match user_found {
        Ok(user) => {
            let projects_found = UserProject::belonging_to(&user)
                            .inner_join(projects::table.on(project_user::idproject.eq(projects::idproject)))
                            .select(Project::as_select())
                            .load::<Project>(connection);
            
            match projects_found {
                Ok(projects) => {
                    let projects_info = project::get_user_projects(projects, connection);
                    let user_projects = UserProjects {
                        projects: projects_info
                    };
                    Ok(user_projects)
                },
                Err(err) => Err(GenericError {error: true, message: err.to_string()})
            }
        }, 
        Err(err) => Err(GenericError {error: true, message: err.to_string()})
    }
}