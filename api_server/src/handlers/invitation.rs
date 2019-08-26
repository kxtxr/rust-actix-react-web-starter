use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, MysqlConnection};
use futures::Future;

use crate::errors::ServiceError;
use crate::models::{invitation::Invitation, Pool};
use crate::services::email::send_invitation;

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

pub fn post_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    // run diesel blocking code
    web::block(move || create_invitation(invitation_data.into_inner().email, pool)).then(
        |res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            },
        },
    )
}

fn create_invitation(
    eml: String,
    pool: web::Data<Pool>,
) -> Result<(), crate::errors::ServiceError> {
    let invitation = dbg!(query(eml, pool)?);
    send_invitation(&invitation)
}

/// Diesel query
fn query(
    email: String,
    pool: web::Data<Pool>,
) -> Result<Invitation, crate::errors::ServiceError> {
    use crate::schema::invitations::dsl::{id, invitations};

    let uuid: String = format!("{}", uuid::Uuid::new_v4());
    dbg!(&uuid);
    let new_invitation: Invitation = Invitation::from_details(uuid, email);
    let conn: &MysqlConnection = &pool.get().unwrap();

    diesel::insert_into(invitations)
        .values(&new_invitation)
        .execute(conn)?;

    let mut items = invitations
        .filter(id.eq(&new_invitation.id))
        .load::<Invitation>(conn)?;
    Ok(items.pop().unwrap())
}
