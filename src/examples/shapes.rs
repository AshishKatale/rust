use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

pub struct Rectangle {
    pub length: f64,
    pub width: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2_f64;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::examples::shapes::Shape;

    use super::{Circle, Rectangle, Triangle};

    #[test]
    fn shape_area() {
        let circle = Circle { radius: 5_f64 };
        let rect = Rectangle {
            length: 5_f64,
            width: 6_f64,
        };
        let trig = Triangle {
            a: 5_f64,
            b: 6_f64,
            c: 7_f64,
        };

        assert_eq!(78.53981633974483_f64, circle.area());
        assert_eq!(30_f64, rect.area());
        assert_eq!(14.696938456699069_f64, trig.area());
    }
}
