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
![Proyecto Integrado DB 11_04_2023](https://github.com/ProyectoIntegradoOrganizationalApp/Back-End/blob/main/Version%202%20Proyecto%20Integrado%20DB%2024_04_2023.jpg)
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

La tabla **User** se relaciona con **Project**. En esta última, la **FK** *idUser*
se refiere al usuario que ha creado el proyecto. Mientras que por otro lado en la tabla **Project_user**, la **FK** *idUser* se refiere a cada uno de los usuarios pertenecientes al proyecto, incluido el creador. De esta manera se mantiene identificado al fundador para cualquier posible función posterior que pueda tener.

#### PROJECT
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**     | `idUser`  |
|      | `name`  |
|      | `configuration (json)`  |

#### PROJECT_USER
| Type           | Field    |
| :--------      | :------- |
| **PF1**         | `idProject`     |
| **PF2**     | `idUser`  |
| **FK3**     | `idRol`  |

En la tabla **Project_user** aparece una nueva **FK** (*idRol*), que pertenece claramente a la tabla **Role**. Los usuarios pueden tener distintos roles (Administrador, Escritor, Lector, ...) en distintos proyectos. Un mismo usuario puede ser administrador de un proyecto y únicamente lector en otro. Es por ello que la *FK* se ve representada en la tabla intermedia **Project_user**.

#### ROLE
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **UNIQUE**     | `Enum (Role)`  |

Los usuarios pueden ser amigos de otros usuarios. Esto se trata de una relación recursiva *many to many*. La forma de representarla es haciendo uso de la tabla **User_friend**, en la que se guardan los ids de los usuarios que tienen amistad entre sí.

#### USER_FRIEND
| Type           | Field    |
| :--------      | :------- |
| **PF1**         | `idUser`     |
| **PF2**     | `idFriend`  |

En todo sitio web los usuarios también pueden dar su opinión y comentar. Es por ello que existe la tabla **Review**, que tiene como **FK** *idUser* puesto que un comentario solo puede pertenecer a un usuario.

#### REVIEW
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**     | `idUser`  |
|     | `title`  |
|    | `content`  |
|   | `rating`  |

Siguiendo con las funcionalidades del usuario, hay que mencionar la tabla **User_invitation**, que se encarga de la invitación de un usuario a otro a un proyecto determinado. Es por ello que posee el id tanto del anfitrión como del invitado, así como del proyecto.

#### USER_INVITATION
| Type           | Field    |
| :--------      | :------- |
| **PF1**         | `idProject`     |
| **PF2**     | `idGuest`  |
| **PF3**     | `idUser`  |
|     | `title`  |
|    | `message`  |

Los logros también son parte de la aplicación, que irá consiguiendo el usuario a medida que vaya cumpliendo los objetivos necesarios para alcanzar cada uno
de ellos. De ahí nacen las tablas **Achievement** y **Achievement_user**. Puesto que un mismo logro puede pertenecer a muchos usuarios y un usuario puede
tener muchos logros, necesitamos una tabla intermedia que recoja tanto las **PK** *idAchievement* e *idUser* además de registrar el progreso de ese usuario 
en ese logro.

#### ACHIEVEMENT
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
|      | `title`  |
|      | `description`  |
|      | `icon`  |
|      | `configuration (json)`  |

#### ACHIEVEMENT_USER
| Type           | Field    |
| :--------      | :------- |
| **PF1**         | `idAchievement` |
| **PF2**     | `idUser`  |
|      | `progress`  |
|      | `completed`  |

El usuario también cuenta con un apartado de notificaciones. Para almacenarlos existe la tabla **Notification**.

#### NOTIFICATION
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**         | `idUser`     |
|      | `title`  |
|      | `content`  |
|      | `state`  |

Finalizada la parte del usuario, pasamos a la del proyecto. Cada proyecto debe de tener unos objetivos a cumplir por los miembros del mismo. La tabla **Goal**
es la encargada de almacenar estos objetivos.

#### GOAL
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**         | `idProject`     |
|      | `name`  |
|      | `description`  |
|      | `completed`  |

Todo proyecto debería de tener una copia de seguridad. De ahí nace la tabla **Recent_change**, la cual almacena una copia del los cambios más recientes para que en caso de fallo no se pierdan los datos. La **PK** en este caso se trata de un campo de tipo fecha. Al tomar como unidad de tiempo hasta las milésimas, no es posible que se realicen dos cambios simultáneamente en el mismo proyecto y en el mismo momento. Es por ello que esta tabla no tiene id. El campo backup sería de tipo **BLOB** o **JSON** (aún por determinar).

#### RECENT_CHANGE
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `date`     |
| **PF**     | `idProject`  |
|     | `backup`  |

Cada proyecto puede tener múltiples aplicaciones, que pueden ser de los siguientes tipos: **Docs**, **Kanban** y **Timeline**.
En estas tablas se encuentran tanto campos comunes como específicos. Es el claro ejemplo de especificación.

#### APP, DOCS, KANBAN, TIMELINE
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **PF**     | `idProject`  |
|     | `...`  |

Las aplicaciones de tipo **Kanban** pueden tener múltiples tableros (**Board**). Estos a su vez pueden tener varias columnas (**Column**) en las que dividir las
tareas a realizar (**Task**). Esto traspasado a tablas quedaría así:

#### BOARD
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**         | `idApp`     |
|      | `title`  |

#### COLUMN
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**         | `idBoard`     |
|      | `title`  |

#### TASK
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**         | `idColumn`     |
|      | `title`  |
|      | `description`  |
|      | `github`  |
|      | `configuration (json)`  |


## ESTRUCTURAS json
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
pub struct UserAchievementsResponse {
    pub total: usize,
    pub achievements: Vec<UserAchievement> // Array de UserAchievement
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

#### Devuelve todos los achievements de un usuario
```http
  GET /profile/<id>
```
| Requires token | Returns     | 
| :-------       | :-------    | 
| Yes            | `UserProfile` o `GenericError` |

| Parameter   | Type          | 
| :--------   | :-------      | 
| `id` | `String`   | 
