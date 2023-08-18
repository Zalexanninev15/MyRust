mod shapes;

fn main() {
    println!("Circle area {}", shapes::Circle { radius: 7. }.get_area());
    println!("Rectangle area {}", shapes::Rectangle::new( 4., 5.).get_area());
}
