pub mod error;
pub mod types;
pub mod models;
pub mod auth;
pub use types::{Status, Visibility};

pub use error::{DError, DResult};

#[macro_export]
macro_rules! test {
    () => { 1 + 3 };

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
