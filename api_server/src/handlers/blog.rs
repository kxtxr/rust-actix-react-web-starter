use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, MysqlConnection};
use futures::Future;

use crate::errors::ServiceError;
use crate::models::{blog::Blog, Pool};

#[derive(Debug, Deserialize)]
pub struct BlogData {
    pub title: String,
    pub content: String,
}

pub fn register_user(
    blog_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || {
        query(
            invitation_id.into_inner(),
            user_data,
            pool,
        )
    })
    .then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    })
}

fn create_blog(
    blog_data: BlogData,
    pool: web::Data<Pool>,
) -> Result<(), crate::errors::ServiceError> {
    let blog = dbg!(query(eml, pool)?);
    send_invitation(&blog)
}

/// Diesel query
fn query(
    blog_data: BlogData,
    pool: web::Data<Pool>,
) -> Result<SlimUser, crate::errors::ServiceError> {
    use crate::schema::invitations::dsl::{id, invitations};
    use crate::schema::users::dsl::{email, users};

    let conn: &MysqlConnection = &pool.get().unwrap();
    invitations
        .filter(id.eq(&invitation_id))
        .load::<Invitation>(conn)
        .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
        .and_then(|mut result| {
            if let Some(invitation) = result.pop() {
                // if invitation is not expired
                if invitation.expires_at > chrono::Local::now().naive_local() {
                    // try hashing the password, else return the error that will be converted to ServiceError
                    let password: String = hash_password(&password)?;
                    dbg!(&password);
                    let user = User::from_details(invitation.email, password);
                    diesel::insert_into(users).values(&user).execute(conn)?;

                    let inserted_user =
                        users.filter(email.eq(&user.email)).first::<User>(conn)?;
                    return Ok(inserted_user.into());
                }
            }
            Err(ServiceError::BadRequest("Invalid Invitation".into()))
        })
}
