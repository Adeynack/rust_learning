fn main() {
    println!("The largest number is: {}",
             largest(&vec![34, 50, 25, 100, 65]));
    println!("The largest number is: {}",
             largest(&vec![102, 34, 6000, 89, 54, 2, 43, 8]));
    println!("The largest number is: {}",
             largest_ref(&vec![102, 34, 6000, 89, 54, 2, 43, 8]));

    let p1 = Point { x: 42, y: 102 };
    println!("Point of i32: {:?}", p1);
    println!("Point of i32: ({}, {})", p1.x(), p1.y());
    let p2 = Point { x: "AAC", y: "BTY" };
    println!("Point of str: {:?}", p2);
    let p3 = Point { x: 453.12454, y: 325.2 };
    println!("Distance of point ({}, {}) to origin is {}",
             p3.x(), p3.y(), p3.distance_from_origin());


    let p11 = PointMix { x: 123, y: 9.876 };
    println!("Point of i32 and f64: {:?}", p11);
    let p12 = PointMix { x: "Hello", y: 'c' };
    let p11str = format!("{:?}", p11); // only way to make the `mixup` line compile with legal borrowing
    let p12str = format!("{:?}", p12); // only way to make the `mixup` line compile with legal borrowing
    let p13 = p11.mixup(p12);
    println!("Mixing points {} and {} yields {:?}", p11str, p12str, p13);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    &largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct PointMix<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointMix<T, U> {
    fn mixup<V, W>(self, other: PointMix<V, W>) -> PointMix<T, W> {
        PointMix {
            x: self.x,
            y: other.y,
        }
    }
}