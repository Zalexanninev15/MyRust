pub struct Rectangle {
    a: f64,
    b: f64
}

impl Rectangle {
    pub fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }

    pub fn get_area(&self) -> f64 {
        self.a * self.b
    }
}