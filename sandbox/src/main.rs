fn main() {
    let num_x = 3;
    let num_y = num_x;
    println!("num_x: {}, num_y: {}", num_x, num_y);

    // error: value borrowed here after move
    // let str_x = String::from("hello");
    // let str_y = str_x;
    // println!("str_x: {}, str_y: {}", str_x, str_y);
}
