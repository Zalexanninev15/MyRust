pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}