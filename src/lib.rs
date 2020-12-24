pub mod db;
pub mod api;
pub mod models;
pub mod error;
pub mod gql;
pub mod middleware;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
