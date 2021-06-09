use std::thread;

use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse};
use log::debug;

use crate::errors::ApiError;
use crate::{services, DbPool};
use crate::services::auth::AuthedUser;


#[post("/refresh")]
pub async fn refresh(db: web::Data<DbPool>, auth: AuthedUser) -> Result<HttpResponse, ApiError> {
    debug!("Refreshing with {}", auth.login);
    thread::spawn(move || services::refresh(&db.into_inner()));

    Ok(HttpResponse::new(StatusCode::ACCEPTED))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(refresh);
}
