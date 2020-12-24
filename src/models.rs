pub mod fact;
pub mod user;
pub mod record;
pub mod item;

pub use self::{
    item::Item,
    fact::Fact,
    user::User,
    record::Record,
};

pub trait Model {

}
