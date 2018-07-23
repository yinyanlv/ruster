use actix_web::{Error,actix::Message};
use chrono::{Utc, NaiveDateTime};
use utils::schema::{themes, comments,saves};
use model::response::{ThemePageListMsgs, ThemeAndCommentsMsgs, BlogLikeMsgs,Msgs,BestPersonMsgs};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable,QueryableByName)]
#[table_name = "themes"]
pub struct Theme {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub theme_status: i32,
    pub title: String,
    pub content: String,
    pub view_count: i32,
    pub comment_count: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="themes"]
pub struct NewTheme<'a> {
    pub user_id: i32,
    pub category_id: i32,
    pub title: &'a str,
    pub content: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Comment {
    pub id: i32,
    pub theme_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="comments"]
pub struct NewComment<'a> {
    pub theme_id: i32,
    pub user_id: i32,
    pub content: &'a str,
    pub created_at: NaiveDateTime,
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Save {
    pub id: i32,
    pub theme_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}
#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="saves"]
pub struct NewSave {
    pub theme_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct ThemeNew {
    pub user_id: i32,
    pub category_name: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct ThemeId {
    pub theme_id: i32,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct ThemeComment {
    pub theme_id: i32,
    pub theme_user_id: i32,
    pub user_id: i32,
    pub comment: String,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct ThemePageList {
    pub page_id : i32,
}
#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct BlogSave {
    pub user_id : i32,
    pub theme_id : i32,
}
#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct BlogLike {
    pub theme_id : i32,
    pub user_id : i32,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct ThemeListResult {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub theme_status: i32,
    pub title: String,
    pub content: String,
    pub view_count: i32,
    pub comment_count: i32,
    pub created_at: NaiveDateTime,
    pub category_name: String,
    pub category_name_cn: String,
    pub username: String,
    pub rtime: String,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct CommentReturn {
    pub id: i32,
    pub theme_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub username: String,
    pub rtime: String,
}
#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct BestPerson;

impl Message for ThemePageList {
    type Result = Result<ThemePageListMsgs, Error>;
}

impl Message for ThemeId {
    type Result = Result<ThemeAndCommentsMsgs, Error>;
}

impl Message for ThemeNew {
    type Result = Result<Msgs, Error>;
}
impl Message for ThemeComment {
    type Result = Result<Msgs, Error>;
}
impl Message for BlogSave {
    type Result = Result<Msgs, Error>;
}
impl Message for BlogLike {
    type Result = Result<BlogLikeMsgs, Error>;
}
impl Message for BestPerson {
    type Result = Result<BestPersonMsgs, Error>;
}

impl Theme {
    pub fn new() -> Theme {
        Theme {
            id: 0,
            user_id: 0,
            category_id: 0,
            theme_status: 0,
            title: "".to_owned(),
            content: "".to_owned(),
            view_count: 0,
            comment_count: 0,
            created_at: Utc::now().naive_utc(),
        }
    }
}

impl ThemeListResult {
    pub fn new() -> ThemeListResult {
        ThemeListResult {
            id: 0,
            user_id: 0,
            category_id: 0,
            theme_status: 0,
            title: "".to_string(),
            content: "".to_string(),
            view_count: 0,
            comment_count: 0,
            created_at: Utc::now().naive_utc(),
            category_name: "".to_string(),
            category_name_cn: "".to_string(),
            username: "".to_string(),
            rtime: "".to_string(),
        }
    }
}

impl CommentReturn {
    pub fn new() -> CommentReturn {
        CommentReturn {
            id: 0,
            theme_id: 0,
            user_id: 0,
            content: "".to_string(),
            created_at: Utc::now().naive_utc(),
            username: "".to_string(),
            rtime: "".to_string(),
        }
    }
}