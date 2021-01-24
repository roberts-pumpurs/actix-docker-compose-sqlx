use crate::{
    api::ApiResult,
    how::Error,
    state::AppState,
    todo::models::users::{IUser, Register},
};
use actix_web::{get, post};
use actix_web::{web, HttpRequest, Responder};

#[get("")]
async fn get_users(_req: HttpRequest, state: web::Data<AppState>) -> Result<impl Responder, Error> {
    println!("HERE");
    let users = &state.user_all().await?;
    debug!("users {:#?}", &users);
    let res = ApiResult::new().with_msg("ok").with_data(users);
    Ok(res.to_resp())
}

#[get("/{id}")]
async fn get_users_single(
    path: web::Path<(u64,)>,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner().0;
    let users = &state.user_query(id).await?;
    debug!("users {:#?}", &users);
    let res = ApiResult::new().with_msg("ok").with_data(users);
    Ok(res.to_resp())
}

#[post("")]
async fn create_users(
    form: web::Json<Register>,
    state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let form = form.into_inner();
    let id = state.user_add(&form).await?;
    let res = ApiResult::new().with_msg("ok").with_data(id);
    Ok(res.to_resp())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(get_users_single);
    cfg.service(create_users);
}

#[cfg(test)]
mod tests {
    use crate::testing;
    use futures::StreamExt;
    use actix_web::{test, web, App};


    #[actix_rt::test]
    async fn test_index_get() {
        let spawned_app = testing::utils::spawn_app().await;
        let mut app = test::init_service(
            App::new()
                .data(spawned_app.state)
                .configure(crate::routes::routes),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/api/v1/todo/user")
            .to_request();
        let mut resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let (bytes, _) = resp.take_body().into_future().await;
        let bytes = bytes.unwrap().unwrap();
        let expected = web::Bytes::from_static(b"{\"code\":200,\"msg\":\"ok\",\"data\":[]}");
        assert_eq!(expected, bytes);
    }
}
