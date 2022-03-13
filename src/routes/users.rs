use actix_web::{get, HttpResponse, post, web};
use log::info;
use serde_json::json;

use entity::sea_orm_active_enums::UserRole;
use entity::users;

use crate::errors::ApiError;
use crate::model::{HttpNewUser, HttpUser, PagedResult, PageParameters};
use crate::model::configuration::ApplicationConfiguration;
use crate::services::auth::AuthenticatedUser;
use crate::services::users::UserService;

#[post("/users")]
async fn new_user(
    new_user: web::Json<HttpNewUser>,
    user_service: web::Data<UserService>,
    user: Option<AuthenticatedUser>,
    configuration: web::Data<ApplicationConfiguration>,
) -> Result<HttpResponse, ApiError> {
    let admin = user.map(|x| x.is_admin()).unwrap_or(false);
    if configuration.allow_account_creation.unwrap_or(false)
        || admin
    {
        info!("Recording new user {:?}", new_user);
        let data = new_user.into_inner();

        if data.role == UserRole::Admin && admin {
            log::debug!("Tried to create a new admin with a non admin user");
            return Ok(HttpResponse::Unauthorized().finish());
        }

        let user = user_service.create_user(&data.username, &data.password, data.role).await?;

        Ok(HttpResponse::Created().json(json!({"id": user.id})))
    } else {
        log::debug!("User creation attempt while it's disabled or creator is not admin");
        Ok(HttpResponse::Unauthorized().finish())
    }
}

#[get("/users")]
async fn list_users(
    user_service: web::Data<UserService>,
    page: web::Query<PageParameters>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    if user.is_admin() {
        info!("Get all users");
        
        let truc = |x: users::Model| -> HttpUser { x.into()}; 
        
        let users: PagedResult<HttpUser> = user_service.list_users(page.get_page(), page.get_size()).await?
            .map_content(truc);
        Ok(HttpResponse::Ok().json(users))
    } else {
        Err(ApiError::unauthorized("You are not allowed to do that"))
    }
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(new_user)
        .service(list_users);
}
