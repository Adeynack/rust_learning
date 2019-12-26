fn main() {
    basic_fn_closure();
    move_closure();
}

fn basic_fn_closure() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

fn move_closure() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);
    //                                    ^ value borrowed here after move

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
