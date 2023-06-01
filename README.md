# BACK-END  / API REST RUST-ROCKET-DIESEL-POSTGRESS 

## GUÍA DE INICIO RÁPIDO
#### Requerimientos:
- [Docker](https://www.docker.com/)
- [Compilador de rust](https://www.rust-lang.org/tools/install)
- diesel_cli para postgreSQL

#### Guía:
1. Una vez clonado el repositorio y teniendo los requerimientos, ponemos a funcionar los dockers
```
  docker compose up
```

2. Ejecutamos las migraciones para crear las tablas
```
  diesel migration setup // Según nuestra experiencia en Windows es necesario en linux no
```
```
  diesel migration run
```

3. Compilamos y ejecutamos el programa
```
  cargo run
```

## DISEÑO DE LA BASE DE DATOS  
![Proyecto Integrado DB 11_04_2023](https://github.com/ProyectoIntegradoOrganizationalApp/Back-End/blob/main/Version%206%20Proyecto%20Integrado%20DB%2001_06_2023.jpg)
Una breve explicación sobre el diseño de la base de datos y las relaciones entre tablas.  

#### USER    
Es el núcleo de la BBDD. De esta tabla dependen muchas otras que se verán más adelante, cuya estructura es: 
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **UNIQUE**     | `email`  |
|                | `password`  |
|                | `name`  |
|                | `lastname`  |
|                | `phone`  |
|                | `created_at`  | 
|                | `updated_at`  |
|                | `level`  |

La tabla **User** guarda la información personal del usuario, así como timestamps y el nivel del usuario en la aplicación.
La tabla **User** se relaciona con **Project**. En esta última, la **FK** *idUser*
se refiere al usuario que ha creado el proyecto. Mientras que por otro lado en la tabla **Project_user**, la **FK** *idUser* se refiere a cada uno de los usuarios pertenecientes al proyecto, incluido el creador. De esta manera se mantiene identificado al fundador para cualquier posible función posterior que pueda tener.

#### PROJECT
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**     | `idUser`  |
|      | `name`  |
|      | `description`  |
|      | `icon`  |
|      | `created_at`  |
|      | `updated_at`  |

En la tabla **Project** es donde guardamos cada proyecto, tiene una **FK** (*idUser*) que se refiere al id del usuario creador del proyecto, también tiene nombre, descripción, icono y timestamps

#### PROJECT_USER
| Type           | Field    |
| :--------      | :------- |
| **PF1**         | `idProject`     |
| **PF2**     | `idUser`  |
| **FK3**     | `idRol`  |

En la tabla **Project_user** aparece una nueva **FK** (*idRol*), que pertenece a la tabla **Role**. Un mismo usuario puede ser administrador de un proyecto y lector en otro. Es por ello que la *FK* se ve representada en la tabla intermedia **Project_user**.

#### PROJECT_USER_ACTIVITY
| Type           | Field    |
| :--------      | :------- |
| **PK,FK**         | `idProject`     |
| **PK,FK**     | `idUser`  |
|      | `date`  |
|      | `commits`  |

En la tabla **Project_user_activity** guardaremos información referiada a la acciones de un usuario en un proyect, esta información la usaremos para mostrar una grafica en el perfil del usuario

#### ROLE
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **UNIQUE**     | `Enum (Role)`  |

En la table **Role** guardamos los distintios tipos de roles que puede tener un usuario (Administrador, Escritor, Lector, ...).

#### USER_FRIEND
| Type           | Field    |
| :--------      | :------- |
| **PF1**         | `idUser`     |
| **PF2**     | `idFriend`  |

Los usuarios pueden ser amigos de otros usuarios. Esto se trata de una relación recursiva *many to many*. La forma de representarla es haciendo uso de la tabla **User_friend**, en la que se guardan los ids de los usuarios que tienen amistad entre sí.

#### USER_FRIEND_INVITATION
| Type           | Field    |
| :--------      | :------- |
| **PK,FK**         | `idGuest`     |
| **PK,FK**     | `idUser`  |
|     | `title`  |
|     | `message`  |

Para ser amigo de un usuario hay que enviarle una invitación, la tabla **User_friend_invitation** se encarga de guardar estas peticiones hasta ser aceptadas o denegados por el usuario al que se ha enviado

#### REVIEW
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**     | `idUser`  |
|     | `title`  |
|    | `content`  |
|   | `rating`  |

En todo sitio web los usuarios también pueden dar su opinión y comentar. Es por ello que existe la tabla **Review**, que tiene como **FK** *idUser* puesto que un comentario solo puede pertenecer a un usuario.

#### USER_INVITATION
| Type           | Field    |
| :--------      | :------- |
| **PF1**         | `idProject`     |
| **PF2**     | `idGuest`  |
| **PF3**     | `idUser`  |
|     | `title`  |
|    | `message`  |

Siguiendo con las funcionalidades del usuario, hay que mencionar la tabla **User_invitation**, que se encarga de la invitación de un usuario a otro a un proyecto determinado. Es por ello que posee el id tanto del anfitrión como del invitado, así como del proyecto.

#### ACHIEVEMENT
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
|      | `title`  |
|      | `description`  |
|      | `icon`  |
|      | `category`
|      | `state`  |

Los logros también son parte de la aplicación, que irá consiguiendo el usuario a medida que vaya cumpliendo los objetivos necesarios para alcanzar cada uno
de ellos. De ahí nace la tablas **Achievement**. Cada logro tiene varios states, un ejemplo de esto sería un achievemnt que sea "Crear proyectos", el primer state sería 1 el segundo 5... siendo estos valores el número de proyectos creados necesarios para avanzar de state.

#### ACHIEVEMENT_USER
| Type           | Field    |
| :--------      | :------- |
| **PF1**         | `idAchievement` |
| **PF2**     | `idUser`  |
|      | `progress`  |
|      | `percentage` |
|      | `current_state` |
|      | `completed`  |

Puesto que un mismo logro puede pertenecer a muchos usuarios y un usuario puede tener muchos logros, necesitamos una tabla intermedia que recoja tanto las **PK** *idAchievement* e *idUser* además de registrar el progreso de ese usuario en ese logro, su state actual y si está completo o no.

#### NOTIFICATION
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**         | `idUser`     |
|      | `title`  |
|      | `content`  |
|      | `state`  |

El usuario también cuenta con un apartado de notificaciones. Para almacenarlos existe la tabla **Notification**.

#### GOAL
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**         | `idProject`     |
|      | `name`  |
|      | `description`  |
|      | `completed`  |

Finalizada la parte del usuario, pasamos a la del proyecto. Cada proyecto debe de tener unos objetivos a cumplir por los miembros del mismo. La tabla **Goal**
es la encargada de almacenar estos objetivos.

#### RECENT_CHANGE
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `date`     |
| **PF**     | `idProject`  |
|     | `backup`  |

Todo proyecto debería de tener una copia de seguridad. De ahí nace la tabla **Recent_change**, la cual almacena una copia de los cambios más recientes para que en caso de fallo no se pierdan los datos. La **PK** en este caso se trata de un campo de tipo fecha. Al tomar como unidad de tiempo hasta las milésimas, no es posible que se realicen dos cambios simultáneamente en el mismo proyecto y en el mismo momento. Es por ello que esta tabla no tiene id. El campo backup sería de tipo **BLOB** o **JSON** (aún por determinar).

### APP
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **PF**     | `idProject`  |
|     | `name`  |
|     | `description`  |
|     | `photo`  |

En la table **App** es donde guardaremos la información de todas las apps de cada proyecto, aquí se guardaran valores generales de cada app, como su nombre, descripción o imagen.

#### TASK_APP & DOCS_APP
| Type           | Field    |
| :--------      | :------- |
| **PK,FK**         | `idApp`     |
| **PK,FK**     | `idProject`  |
|     | `type`  |
|     | `...`  |

Tanto la tabla **Task_app** como la **Docs_app** representan tipos generales de apps que pueden haber, esta tiene una **FK** *idApp* que la relaciona con app, siendo que cuando se crea un registro en la tabla **App** se crea uno ya sea en **task_app** o en **docs_app** indicando así su tipo general.
Dentro de estas tablas tenemos un parametro en común que es el tipo de app, por ejemplo en **Task_app** podemos tener una app de tipo *Kanban* o de tipo *Timeline*

#### BOARD - TASK_APP
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**         | `idApp`     |
|      | `title`  |

La tabla **Board** representa una pizarra en el tipo de aplicación general task_app

#### COLUMN - TASK_APP
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**         | `idBoard`     |
|      | `title`  |

La tabla **Column** representa una columna dentro de una pizarra en el tipo de aplicación general task_app

#### TASK - TASK_APP
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**         | `idColumn`     |
|      | `title`  |
|      | `description`  |
|      | `github`  |
|      | `configuration (json)`  |

La tabla **Task** representa una tarea dentro de una columna en el tipo de aplicación general task_app

## ESTRUCTURAS USADAS EN LAS RUTAS
```
GenericError {
    error: bool,
    message: String
}
```
```
UserInput {
    first_name: String,
    last_name: String,
    email: String,
    password: String
}
```
```
UserLogin {
    email: String,
    password: String
}
```
```
UserLoginResponse {
    id: String,
    full_name: String,
    _token: String,
    email: String
}
```
```
ResponseMessage {
    message: String
}
```
```
ChangePass {
    mail: String,
    pass: String,
    confirm_pass: String
}
```
```
 Achievement {
    id: String,
    title: String,
    description: String,
    icon: String
}
```
```
AllAchievementsResponse {
    total: usize,
    achievements: Vec<Achievement> // Array de Achievement
}
```
```
UserAchievement {
    idachievement: String,
    iduser: String,
    progress: i16,
    completed: bool,
}
```
```
UserAchievementsResponse {
    total: usize,
    achievements: Vec<UserAchievement> // Array de UserAchievement
}
```

```
UserProfile {
    user: UserInfoResponse,
    achievements: Vec<UserAchievementsProfile>,
    projects: Vec<UserProjectProfile>,
    activity: Vec<UserActivityProfile>,
    owner : bool
}
```

```
UserAchievementsProfile {
    id: String,
    title: String,
    description: String,
    icon: String,
    progress: i32,
    completed: bool,
    current_state: i32,
    percentage: BigDecimal
}
```

```
UserProjectProfile {
    id: String,
    name: String,
    description: String,
    updated_at: String,
    members: Vec<ProjectMembers>
}
```

```
UserActivityProfile {
    idproject: String,
    date: String,
    commits: i16
}
```

```
InvitationMessage {
    title: String,
    message: String
}
```

```
NewRole {
    idrole: String
}
```

```
ProjectInputCreate {
    name: String,
    description: String
}
```

```
ProjectDetail {
    idproject: String,
    iduser: String,
    name: String,
    description: String,
    created_at: String,
    updated_at: String,
    members: Vec<ProjectMembers>,
    owner: bool
}
```

```
App {
    id: String,
    idproject: String,
    name: String,
    description: String,
    photo: String
}
```

```
AppInputCreate {
    name: String,
    description: String,
    pub photo: String,
    pub apptype: String,
    pub task_app: Option<TaskAppInputCreate>,
    pub docs_app: Option<DocsAppInputCreate>,
}
```

```
TaskAppInputCreate {
    app_type: String
}
```

```
DocsAppInputCreate {
    app_type: String
}
```

```
Board {
    id: String,
    idapp: String,
    title: String
}
```

```
BoardInputCreate {
    idapp: String,
    title: String
}
```

```
Columna {
    id: String,
    idboard: String,
    title: String
}
```

```
ColumnInputCreate {
    idboard: String,
    title: String
}
```

```
Task {
    id: String,
    idcolumn: String,
    title: String,
    description: Option<String>,
    github: Option<String>
}
```

```
TaskInputCreate {
    idcolumn: String,
    title: String,
    description: Option<String>,
    github: Option<String>
}
```
## RUTAS DE LA API

#### Registrar a un usuario
```http
  POST /register
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| No             | `User` o `GenericError`|

| Parameter   | Type          |
| :--------   | :-------      |
| `user_info` | `UserInput`   |

#### Actualizar un usuario
```http
  PUT /user/<id>
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| Yes             | `GenericError` o `GenericError`|

| Parameter   | Type          |
| :--------   | :-------      |
| `user_info` | `UserInput`   |

#### Eliminar un usuario
```http
  DELETE /user/<id>
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| Yes             | `GenericError` o `GenericError`|

#### Iniciar la sesión de un usuario
```http
  POST /login
```
| Requires token | Returns     |
| :-------       | :-------    |
| No             | `UserLoginResponse` o `GenericError`| 

| Parameter   | Type          |
| :--------   | :-------      |
| `user_info` | `UserLogin`   |

#### Enviar un correo a un usuario
```http
  POST /send_mail
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| No             | `ResponseMessage` o `GenericError`

| Parameter   | Type          |
| :--------   | :-------      |
| `user_mail` | `String`   |

#### Cambia la conraseña del usuario
```http
  POST /change_password
```
| Requires token | Returns     |
| :-------       | :-------    |
| No             | `ResponseMessage` o `GenericError`|

| Parameter   | Type          | 
| :--------   | :-------      | 
| `user_info` | `ChangePass`   | 

#### Desloguear al usuario
```http
  POST /logout
```
| Requires token |
| :-------       |
| Yes            |

#### Devuelve todos los achievements de la base de datos
```http
  GET /achievements
```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `AllAchievementsResponse` o `GenericError` |


#### Devuelve todos los achievements de un usuario
```http
  GET /profile/<id>/achievements
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| Yes            | `UserAchievementsResponse` o `GenericError` |

#### Devuelve datos del perfil del usuario
```http
  GET /profile/<id>
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| Yes            | `UserProfile` o `GenericError` |

#### Crear un proyecto
```http
  POST /project
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| Yes            | `Project` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `project_info` | `ProjectInputCreate` |

#### Actualizar un proyecto
```http
  PUT /project/<id>
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| Yes            | `GenericError` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `project_info` | `ProjectInputCreate` |

#### Eliminar un proyecto
```http
  DELETE /project/<id>
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| Yes            | `GenericError` o `GenericError` |

#### Conseguir los datos de un proyecto
```http
  GET /project/<id>
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| Yes            | `ProjectDetail` o `GenericError` |

#### Conseguir todos los proyectos de un usuario
```http
  GET /user/<id>/projects
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| Yes            | `UserProjects` o `GenericError` |

#### Inivar usuario a un proyecto
```http
  POST /user/<user_id>/project/<project_id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `invitation` | `InvitationMessage`   |

#### Aceptar la invitación a un proyecto
```http
  GET /invitation/<project_id>/accept
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

#### Denegar la invitación a un proyecto
```http
  GET /invitation/<project_id>/deny
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

#### Cambiar el rol de un usuario en un proyecto
```http
  PUT /user/<user_id>/project/<project_id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `role` | `NewRole`   |

#### Eliminar un usuario de un proyecto
```http
  DELETE /user/<user_id>/project/<project_id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

#### Enviar petición de amistad a un usuario
```http
  POST /friend/<guest_id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `invitation` | `InvitationMessage`   |

#### Aceptar petición de amistad de usuario
```http
  GET /friend/<user_id>/accept
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

#### Denegar petición de amistad de usuario
```http
  GET /friend/<user_id>/deny
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

#### Eliminar amigo
```http
  DELETE /friend/<friend_id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

#### Añadir un app de un proyecto
```http
  POST /project/<project_id>/app
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `App` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `app_info` | `AppInputCreate` |

#### Actualizar una app de un proyecto
```http
  PUT /project/<project_id>/app
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `app_info` | `AppInputCreate` |

#### Eliminar una app de un proyecto
```http
  DELETE /project/<project_id>/app
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

#### Añadir una pizarra a una app tipo task_app
```http
  POST /task_app/board
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `Board` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `board_info` | `BoardInputCreate` |

#### Actualizar una pizarra de una app tipo task_app
```http
  PUT /task_app/board/<id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `board_info` | `BoardInputCreate` |

#### Eliminar una pizarra de una app tipo task_app
```http
  DELETE /task_app/board/<id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

#### Añadir una columna a una pizarra
```http
  POST /task_app/column
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `Column` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `column_info` | `ColumnInputCreate` |

#### Actualizar un columna de una pizarra
```http
  PUT /task_app/column/<id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `column_info` | `ColumnInputCreate` |

#### Eliminar una columna de una pizarra
```http
  DELETE /task_app/column/<id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

#### Añadir una terea a una columna
```http
  POST /task_app/task
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `Task` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `task_info` | `TaskInputCreate` |

#### Actualizar un tarea de una column
```http
  PUT /task_app/task/<id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `task_info` | `TaskInputCreate` |

#### Eliminar una tarea de una columna
```http
  DELETE /task_app/task/<id>
 ```
| Requires token | Returns       |
| :-------       | :---------    |
| Yes            | `GenericError` o `GenericError` |
