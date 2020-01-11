use std::slice;

static HELLO_WORLD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
        dangerous();
    }

//    let address = 0x0123456usize;
//    let r = address as *const i32; // what is at address `address` in RAM
//    unsafe {
//        println!("r =  {}", *r); // dereference it will most probably seg-fault
//    }

    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    let (left, right) = split_at_mut(&mut arr, 3);
    println!("{:?} <> {:?}", left, right);

    unsafe {
        let a = -3;
        let abs_a = abs(a);
        println!("Absolute value of {} according to C: {}", a, abs_a);
    }

    println!("name is: {}", HELLO_WORLD);

    unsafe { println!("counter = {}", COUNTER); }
    add_to_count(1);
    unsafe { println!("counter = {}", COUNTER); }
    add_to_count(3);
    unsafe { println!("counter = {}", COUNTER); }
    add_to_count(123);
    unsafe { println!("counter = {}", COUNTER); }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn dangerous() {
    let s = "The girl is soooo dannnnngerous";
    let rs = &s as *const &str;
    println!("{}", *rs);
}

// example of safe-wrapping an unsafe function
// note: split_at_mut exists already in the standard lib.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);

    // This would fail because borrowing twice.
    // let left_part = &mut slice[..mid];
    // let right_part = &mut slice[mid..];

    let ptr = slice.as_mut_ptr();
    unsafe {
        let left_part = slice::from_raw_parts_mut(ptr, mid);
        let right_part = slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid);
        (left_part, right_part)
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
