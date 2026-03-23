use std::f64::consts::PI;

enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle { radius } => radius.powi(2) * PI,
        }
    }
}

fn main() {
    let rectangle = Shape::Rectangle {
        width: 15.4,
        height: 75.4,
    };
    let circle = Shape::Circle { radius: 42.0 };

    assert_eq!(rectangle.area(), 1161.16);
    assert_eq!(circle.area(), 5541.769440932395);
    println!("Success!")
}

#[test]
fn test_area() {
    let rectangle = Shape::Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let circle = Shape::Circle { radius: 10.0 };

    assert_eq!(rectangle.area(), 200.0);
    assert_eq!(circle.area(), 314.1592653589793);
}
