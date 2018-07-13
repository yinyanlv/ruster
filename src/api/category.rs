use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::future::Future;

use api::index::AppState;
use model::category::{Categorys, CategoryNew, CategoryThemePageList};

pub fn category_new((category_new, state): (Json<CategoryNew>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(CategoryNew{ 
            user_id: category_new.user_id,
            category_name: category_new.category_name.clone(),
            category_name_cn: category_new.category_name_cn.clone(),
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn categorys(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(Categorys)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn category_theme_page_list((category_theme_page_list, state): (Json<CategoryThemePageList>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(CategoryThemePageList{
            page_id: category_theme_page_list.page_id,
            category_name: category_theme_page_list.category_name.clone(),
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}
