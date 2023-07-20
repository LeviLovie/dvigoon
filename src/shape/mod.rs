pub struct Circle {
    x: i32,
    y: i32,
    radius: i32,
}
pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}
impl Shape {
    pub fn new_circle() -> Shape {
        return Shape::Circle(Circle::new());
    }
    pub fn new_rectrange() -> Shape {
        return Shape::Rectangle(Rectangle::new());
    }
}

trait ShapeBehavior {
    fn new() -> Self;
}
impl ShapeBehavior for Circle {
    fn new() -> Circle {
        println!("Circle::new()");
        return Circle {
            x: 0,
            y: 0,
            radius: 0,
        }
    }
}
impl ShapeBehavior for Rectangle {
    fn new() -> Rectangle {
        println!("Rectangle::new()");
        return Rectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}
