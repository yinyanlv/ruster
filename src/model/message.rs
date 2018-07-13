use chrono::{Utc, NaiveDateTime};
use utils::schema::messages;

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable,Identifiable)]
#[table_name = "messages"]
pub struct Message {
    pub id: i32,
    pub theme_id: i32,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub content: String,
    pub types: i32,
    pub message_status: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="messages"]
pub struct NewMessage<'a> {
    pub theme_id: i32,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub content: &'a str,
    pub created_at: NaiveDateTime,
}