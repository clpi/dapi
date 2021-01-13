pub mod models;
pub mod auth;

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
