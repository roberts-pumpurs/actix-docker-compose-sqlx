use crate::{api::ApiResult, how::Error, state::AppState, todo::models::users::IUser};
use actix_web::{web, HttpRequest, Responder};

pub async fn get_users(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let users = &state.user_query("").await?;
    debug!("users {:#?}", &users);
    let res = ApiResult::new().with_msg("ok").with_data(users);
    Ok(res.to_resp())
}
