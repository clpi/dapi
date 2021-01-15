use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

pub trait Datatype {

}

pub enum Visibility {

}

#[derive(Serialize, Deserialize)]
pub enum ValueKind {
    Integer(i32),
    Double(f32),
    Text(String),
    Datetime(chrono::DateTime<Utc>)
    Enumeration(UserEnum),
}

#[derive(Serialize, Deserialize)]
pub struct UserEnumSelection {
    sel_indices: std::collections::HashMap<bool, UserEnumVal>,
    // user_enum: UserEnum,
}

#[derive(Serialize, Deserialize)]
pub struct UserEnumVal {
    val: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserEnum {
    values: Vec<UserEnumVal>,
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Active,
    Archived,
    Deleted,
    Completed
}

impl Datatype for Visibility {

}

impl Datatype for ValueKind {

}

