use uuid::Uuid;

pub struct IdGenerator;

impl IdGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create(&self) -> Uuid {
        Uuid::now_v7()
    }

}