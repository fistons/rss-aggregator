use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse};
use actix_xml::Xml;
use serde_json::json;

use crate::errors::ApiError;
use crate::model::opml::Opml;
use crate::model::{HttpNewChannel, PageParameters};
use crate::services::auth::AuthenticatedUser;
use crate::startup::ApplicationServices;

#[get("/channel/{id}")]
#[tracing::instrument(skip(services), level = "debug")]
pub async fn get_channel(
    id: web::Path<i32>,
    services: web::Data<ApplicationServices>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let res = services
        .channel_service
        .select_by_id_and_user_id(id.into_inner(), user.id)
        .await?;

    match res {
        Some(data) => Ok(HttpResponse::Ok().json(data)),
        None => Ok(HttpResponse::new(StatusCode::NOT_FOUND)),
    }
}

#[get("/channels")]
#[tracing::instrument(skip(services), level = "debug")]
pub async fn get_channels(
    services: web::Data<ApplicationServices>,
    page: web::Query<PageParameters>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let channels = services
        .channel_service
        .select_page_by_user_id(user.id, page.get_page(), page.get_size())
        .await?;
    Ok(HttpResponse::Ok().json(channels))
}

#[post("/channels")]
#[tracing::instrument(skip(services), level = "debug")]
async fn new_channel(
    new_channel: web::Json<HttpNewChannel>,
    services: web::Data<ApplicationServices>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let data = new_channel.into_inner();
    let channel = services
        .channel_service
        .create_or_link_channel(data, user.id)
        .await?;

    Ok(HttpResponse::Created().json(json!({"id": channel.id})))
}

#[get("/channel/{chan_id}/items")]
#[tracing::instrument(skip(services), level = "debug")]
async fn get_items_of_channel(
    chan_id: web::Path<i32>,
    page: web::Query<PageParameters>,
    services: web::Data<ApplicationServices>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let items = services
        .item_service
        .get_items_of_channel(
            chan_id.into_inner(),
            auth.id,
            page.get_page(),
            page.get_size(),
        )
        .await?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("/channels/import")]
#[tracing::instrument(skip(services, opml), level = "debug")]
async fn import_opml(
    services: web::Data<ApplicationServices>,
    auth: AuthenticatedUser,
    opml: Xml<Opml>,
) -> Result<HttpResponse, ApiError> {
    let opml = opml.into_inner();
    for channel in opml.body.flatten_outlines() {
        services
            .channel_service
            .create_or_link_channel(channel, auth.id)
            .await?;
    }

    Ok(HttpResponse::Created().finish())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_channel)
        .service(get_channels)
        .service(new_channel)
        .service(get_items_of_channel)
        .service(import_opml);
}
