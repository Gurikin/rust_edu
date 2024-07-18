fn main() {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference
    let r_abs2 = r.abs(); // implicit dereference
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit dereference
    let s_len2 = s.len(); // implicit dereference
    assert_eq!(s_len1, s_len2);

    let a = Box::new(15);
    let b = Box::new(&a);
    // for copy value from a (digit 15) we need 3 dereference
    let mut c = ***b;
    c += 1;
    println!("dereferenced value c = {}", c);
    print_type_of(&c);
    println!("value a = {}", a);
}

fn print_type_of<T>(_: &T) {
    println!("type of value = {}", std::any::type_name::<T>())
}
