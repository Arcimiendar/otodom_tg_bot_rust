use chrono::NaiveDateTime;
use super::schema::appartment;
use super::schema::user;

#[derive(Debug, Queryable)]
pub struct Appartment {
    pub id: i32,
    pub price: Option<i32>,
    pub czynsz: Option<i32>,
    pub name: Option<String>,
    pub rooms: Option<i32>,
    pub scrapped_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[table_name="appartment"]
pub struct NewAppartment {
    pub price: Option<i32>,
    pub czynsz: Option<i32>,
    pub name: Option<String>,
    pub rooms: Option<i32>,
    pub scrapped_at: Option<NaiveDateTime>,
}


#[derive(Debug, Queryable, Insertable)]
#[table_name="user"]
pub struct User {
    pub id: i32
}