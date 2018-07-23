use actix_web::{Error,actix::Message};
use utils::schema::categorys;
use chrono::{Utc, NaiveDateTime};
use model::response::{CategorysMsgs, Msgs, ThemePageListMsgs};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Category {
    pub id: i32,
    pub user_id: i32,
    pub category_name: String,
    pub category_name_cn: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="categorys"]
pub struct NewCategory<'a> {
    pub user_id: i32,
    pub category_name: &'a str,
    pub category_name_cn: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CategoryNew {
    pub user_id: i32,
    pub category_name: String,
    pub category_name_cn: String,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct Categorys;

#[derive(Deserialize,Serialize, Debug, Clone)]
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
