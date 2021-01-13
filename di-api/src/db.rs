#[derive(Clone)]
pub struct Database {
    db_url: String,
}

impl Database {
    fn new() -> Database {
        Self { db_url: String::new() }
    }

    fn insert(&self) -> u64 {
        0
    }

    pub async fn get<I: Insertable>(&mut self, item: I) -> async_std::io::Result<()> {
        Ok(())
    }

    pub async fn set<I: Insertable>(&mut self, iid: usize, item: I) -> async_std::io::Result<()> {
        Ok(())
    }
}

pub trait Insertable {

}
