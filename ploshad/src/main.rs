// 1. Определяем трейт Shape с методами площади и периметра
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// 2. Реализуем Shape для треугольника
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Формула Герона: S = √(p(p-a)(p-b)(p-c)), где p = (a+b+c)/2
        let p = self.perimeter() / 2.0;
        (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

// 3. Реализуем Shape для прямоугольника
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// 4. Реализуем Shape для круга
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// 5. Функция, которая считает отношение периметра к площади
fn perimeter_to_area(shape: &impl Shape) -> f64 {
    shape.perimeter() / shape.area()
}

fn main() {
    // Проверяем работу на примерах:
    let triangle = Triangle { a: 3.0, b: 4.0, c: 5.0 };
    let rectangle = Rectangle { width: 2.0, height: 3.0 };
    let circle = Circle { radius: 2.0 };

    println!("Triangle P/A: {}", perimeter_to_area(&triangle)); // Ожидается ~2.0
    println!("Rectangle P/A: {}", perimeter_to_area(&rectangle)); // Ожидается ~1.666...
    println!("Circle P/A: {}", perimeter_to_area(&circle)); // Ожидается 1.0
}