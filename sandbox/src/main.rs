fn main() {
    // let num_x = 3;
    // let num_y = num_x;
    // println!("num_x: {}, num_y: {}", num_x, num_y);

    // // error: value borrowed here after move
    // // let str_x = String::from("hello");
    // // let str_y = str_x;
    // // println!("str_x: {}, str_y: {}", str_x, str_y);

    // let str_a = String::from("hello");
    // let str_b = str_a.clone();
    // println!("str_a: {}, str_b: {}", str_a, str_b);

    let s = String::from("hello");
    print!("Before function call: {}\n", s);
    takes_ownership(s);
    // error: value borrowed here after move
    // print!("After function call: {}\n", s);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}