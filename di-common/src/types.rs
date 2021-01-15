use std::str::FromStr;
use crate::error::DError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use dynomite::{FromAttributes, AttributeValue, Attribute, dynamodb, DynamoDbExt, Attributes, AttributeError};

#[async_trait::async_trait]
pub trait Datatype: Attribute {

    async fn get(&self) -> Result<(), dynamodb::PutItemError> {
        Ok(())
    }

    async fn entries_with(&self, table: &str) -> Result<(), dynamodb::PutItemError> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
pub enum Visibility {
    Public,
    Private,
    FriendsOnly,
    // Selected(Vec<String>),
}

// #[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
// pub enum ValueKind {
//     Integer(i32),
//     Double(f32),
//     Text(String),
//     Datetime(chrono::DateTime<Utc>),
    // Date(chrono::NaiveDate),
    // Duration(std::time::Duration, Option<DateTime<Utc>>),
    // Enumeration(UserEnum),
// }

// #[derive(Serialize, Deserialize, Clone, Debug,)]
// pub struct UserEnumSelection {
//     sel_indices: std::collections::HashMap<bool, UserEnumVal>,
//     // user_enum: UserEnum,
// }

// #[derive(Serialize, Deserialize)]
// pub struct UserEnumVal {
//     val: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct UserEnum {
//     values: Vec<UserEnumVal>,
// }

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
pub enum Status {
    Active,
    Archived,
    Deleted,
    Completed
}

impl Status {
    pub fn active() -> Self {
        Self::Active
    }
}

impl Visibility {
    pub fn public() -> Self {
        Self::Public
    }

    pub fn private() -> Self {
        Self::Private
    }
}

impl Datatype for Visibility {

}

impl std::str::FromStr for Visibility {
    type Err = DError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "public" => Ok(Self::Public),
            "private" => Ok(Self::Private),
            "friends_only" => Ok(Self::FriendsOnly),
            _ => Err(DError::AttError(AttributeError::InvalidFormat)),
        }
    }
}

impl ToString for Visibility {
    fn to_string(&self) -> String {
        match self {
            Self::FriendsOnly => "friends_only".to_string(),
            Self::Private => "private".to_string(),
            Self::Public => "public".to_string(),
            // Self::Selected(sel) => sel.join(" ").to_string(),
        }
    }
}

impl Attribute for Visibility {

    fn from_attr(value: AttributeValue) -> Result<Self, AttributeError> {
        if let Some(s) = value.s {
            match Visibility::from_str(s.as_str()) {
                Ok(v) => Ok(v),
                Err(_) => Err(AttributeError::InvalidFormat),
            }
        // } else if let Some(ss) = value.ss {
        //     Ok(Self::Selected(ss))
        } else {
            Err(AttributeError::InvalidFormat)
        }
    }

    fn into_attr(self: Self) -> AttributeValue {
        match self {
            // Self::Selected(sel) => {
            //     AttributeValue { ss: Some(sel), ..Default::default() }
            // },
            _ => AttributeValue { s: Some(self.to_string()), ..Default::default() }
        }

    }
}
