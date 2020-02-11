use diesel;
use diesel::prelude::*;
use diesel::pg::Pgconnection;

#[#[derive(Queryable)]]
pub struct Users{
    pub id: i32,
    pub name: String,
    pub username: String,
    pub active: bool

}

#[#[derive(Insertable)]]
#[table_name="Users"]
pub struct NewUser{
    pub id: i32,
    pub name: String,
    pub username: String,
    pub active: bool

}

