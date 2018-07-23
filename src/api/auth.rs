use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::Future;
use router::AppState;
use model::user::{SignupUser, SigninUser};

pub fn signup((signup_user, state): (Json<SignupUser>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(SignupUser{
        username: signup_user.username.clone(),
        email: signup_user.email.clone(),
        password: signup_user.password.clone(),
        confirm_password: signup_user.confirm_password.clone(),
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(signup_msg) => Ok(HttpResponse::Ok().json(signup_msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn signin((signin_user, state): (Json<SigninUser>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(SigninUser{
        username: signin_user.username.clone(),
        password: signin_user.password.clone(),
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(signin_msg) => Ok(HttpResponse::Ok().json(signin_msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}