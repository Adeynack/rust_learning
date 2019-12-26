fn main() {
    //
    // Version with 2 parameters
    //
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels. (2 params)",
        area_with_2_parameterss(width1, height1)
    );

    //
    // Version with tuples
    //
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels. (tuple)",
        area_with_tuple(rect1)
    );

    //
    // Version with struct
    //
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels. (struct)",
        area_with_struct(&rect2)
    );

    //
    // Printing the rectangle
    //
    println!("The rectangle is {:?}", rect2);
    println!("The rectangle is {:#?}", rect2);
}

fn area_with_2_parameterss(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuple(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
