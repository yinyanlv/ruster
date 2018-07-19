use actix::*;
use actix_web::*;
use utils::schema::categorys;
use model::db::ConnDsl;
use chrono::{Utc, NaiveDateTime};
use model::response::{CategorysMsgs, Msgs, ThemePageListMsgs};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i32,
    pub user_id: i32,
    pub category_name: String,
    pub category_name_cn: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="categorys"]
#[serde(rename_all = "camelCase")]
pub struct NewCategory<'a> {
    pub user_id: i32,
    pub category_name: &'a str,
    pub category_name_cn: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryNew {
    pub user_id: i32,
    pub category_name: String,
    pub category_name_cn: String,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct Categorys;

#[derive(Deserialize,Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryThemePageList {
    pub page_id: i32,
    pub category_name: String,
}

impl Message for CategoryNew {
    type Result = Result<Msgs, Error>;
}

impl Message for Categorys {
    type Result = Result<CategorysMsgs, Error>;
}

impl Message for CategoryThemePageList {
    type Result = Result<ThemePageListMsgs, Error>;
}
