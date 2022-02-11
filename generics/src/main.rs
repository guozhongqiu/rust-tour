struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl<T, U> Point2<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

struct Point3<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point3<T, U> {
    fn mixup<V, W>(self, other: Point3<V, W>) -> Point3<T, W> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    //point and point2
    let p = Point { x: 5, y: 10 };
    let p2 = Point2 { x: 5.5, y: 10 };

    println!("p.x = {}, p.y = {}, p2.x = {}, p2.y = {}",
        p.x(), p.y(), p2.x(), p2.y());

    // point3
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

