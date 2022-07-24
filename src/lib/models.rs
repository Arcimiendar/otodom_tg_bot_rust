use std::fmt;
use std::fmt::Formatter;
use chrono::{NaiveDateTime, Utc};
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

fn ref_unwrap_or<'a, T>(option: &'a Option<T>, default: &'a T) -> &'a T {
    option.as_ref().unwrap_or(default)
}

impl fmt::Display for Appartment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", vec![
            format!("{}", ref_unwrap_or(&self.name, &String::from(""))),
            format!("price: {}", ref_unwrap_or(&self.price, &0)),
            format!("czynsz: {}", ref_unwrap_or(&self.czynsz, &0)),
            format!("rooms: {}", ref_unwrap_or(&self.rooms, &0)),
            format!(
                "when scrapped: {}",
                ref_unwrap_or(&self.scrapped_at, &Utc::now().naive_utc())
                    .format("%Y-%m-%d %H:%M:%S").to_string()
            )
        ].join("\n"))
    }
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