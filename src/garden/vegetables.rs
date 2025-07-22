#[derive(Debug)]
pub struct Asparagus {
    name: String
}

impl Asparagus {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn test(&self) {
        println!("test Asparagus name : {}", self.name);
    }
}