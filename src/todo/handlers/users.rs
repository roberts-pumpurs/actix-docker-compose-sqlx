use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
// use super::models::users::IUser;

use crate::{how::Error, state::AppState, todo::models::users::IUser};

pub async fn get_users(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let users = &state.user_query("").await?;
    debug!("users {:#?}", &users);
    Ok(HttpResponse::Ok().body("Hey there!"))
}
