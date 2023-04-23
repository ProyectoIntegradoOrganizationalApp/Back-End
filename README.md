## BACK-END  / API REST RUST-ROCKET-DIESEL-POSTGRESS 

#### DISEÑO DE LA BASE DE DATOS  
![Proyecto Integrado DB 11_04_2023](https://github.com/ProyectoIntegradoOrganizationalApp/Back-End/blob/main/Proyecto%20Integrado%20DB%2011_04_2023.jpg)
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

La tabla **User** se relaciona con **Project**. En esta última, la **FK** *idUser*
se refiere al usuario que ha creado el proyecto. Mientras que por otro lado en la tabla **Project_user**, la **FK** *idUser* se refiere a cada uno de los usuarios pertenecientes al proyecto, incluido el creador. De esta manera se mantiene identificado al fundador para cualquier posible función posterior que pueda tener.

#### PROJECT
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **FK**     | `idUser`  |
|      | `name`  |

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
|    | `message`  |
|   | `rating`  |

Para terminar con las funcionalidades del usuario, hay que mencionar la tabla **User_invitation**, que se encarga de la invitación de un usuario a otro a un proyecto determinado. Es por ello que posee el id tanto del anfitrión como del invitado, así como del proyecto.

#### USER_INVITATION
| Type           | Field    |
| :--------      | :------- |
| **PF1**         | `idProject`     |
| **PF2**     | `idGuest`  |
| **PF3**     | `idUser`  |
|     | `title`  |
|    | `message`  |

Todo proyecto debería de tener una copia de seguridad. De ahí nace la tabla **Recent_change**, la cual almacena una copia del los cambios más recientes para que en caso de fallo no se pierdan los datos. La **PK** en este caso se trata de un campo de tipo fecha. Al tomar como unidad de tiempo hasta las milésimas, no es posible que se realicen dos cambios simultáneamente en el mismo proyecto y en el mismo momento. Es por ello que esta tabla no tiene id. El campo backup sería de tipo **BLOB** o **JSON** (aún por determinar).

#### RECENT_CHANGE
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `date`     |
| **PF**     | `idProject`  |
|     | `backup`  |

Cada proyecto puede tener múltiples aplicaciones, que pueden ser de los siguientes tipos: **Docs**, **Kanban** y **Timeline**.
En estas tablas se encuentran tanto campos comunes como específicos. Es el claro ejemplo de especificación. (Los campos tanto de la app base como de las especificaciones aún están por definir, hay que comentarlos en la reunión).

#### APP, DOCS, KANBAN, TIMELINE
| Type           | Field    |
| :--------      | :------- |
| **PK**         | `id`     |
| **PF**     | `idProject`  |
|     | `...`  |
