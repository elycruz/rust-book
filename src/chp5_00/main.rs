#[derive(Debug)]
struct Color (u32, u32, u32);

#[derive(Debug)]
struct Point (i32, i32);

#[derive(Debug)]
struct Rectangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}

fn main () {
    let rect1 = Rectangle{x: 0, y: 0, w: 100, h: 100};
    let rect2 = Rectangle{x: 0, y: 0, w: 100, h: 100};
    let rect2 = Rectangle{x: 0, y: 0, w: 100, h: 200};
    let color = Color(0,0,0);
    let point = Point(0,0);
    println!("Rectangle {:?}", rect1);
    println!("Color {:?}", color);
    println!("Point {:?}", point);
    println!("\nRectangle area: {:?}", rect1.rect_area());
    println!("\nRect1 can contain rect2: {:?}", rect1.can_contain(&rect2));
    println!("\nRect1 can contain rect2: {:?}", rect1.can_contain(&rect3));
}

impl Rectangle {
    fn rect_area (&self) -> i32 {
        let out = (self.w - self.x) * (self.h - self.y);
        if out < 0 { out * -1 } else { out }
    }

    fn can_contain (&self, other: &Rectangle) -> bool {
        let a1 = self.rect_area();
        let a2 = other.rect_area();
        a1 >= a2
    }
}
