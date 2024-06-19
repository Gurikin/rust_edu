fn move_a_box(b: Box<i32>) {
    // Moving already done
}

fn main() {
    ownership_v2();
}

fn ownership_v1() {
    let b = Box::new(0);
    let b2 = b;
    move_a_box(b);
}

/// It is wrong quiz data
/// compiler think that it is undefined behaviour
/// but quiz says that it is not
fn ownership_v2() {
    let b = Box::new(0);
    let b2 = b;
    println!("{}", b);
    move_a_box(b2);
}

fn ownership_v3() {
    let b = Box::new(0);
    move_a_box(b);
    let b2 = b;
}

fn ownership_v4() {
    let b = Box::new(0);
    move_a_box(b);
    println!("{}", b);
}
