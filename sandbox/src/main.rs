use std::collections::HashMap;

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

    let ss = String::from("world");
    print!("Before function call: {}\n", ss);
    let sss = takes_and_gives_back(ss);
    print!("After function call: {}\n", sss);

    let mut v: Vec<i32> = Vec::new();
    println!("v is {:?}", v);

    v.push(1);
    v.push(2);
    println!("v is {:?}", v);

    let val = v.get(0);
    println!("val is {:?}", val);

    let val = v.get(100);
    println!("val is {:?}", val);

    for e in &v {
        println!("{}", e);
    }

    for e in &mut v {
        *e *= 2;
    }
    println!("v is {:?}", v);

    let mut map = HashMap::new();
    map.insert(String::from("hello"), 1);
    map.insert(String::from("world"), 2);
    println!("map is {:?}", map);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let team_scores: Vec<_> = teams.iter().zip(initial_scores.iter()).collect();
    println!("team_scores is {:?}", team_scores);

    let first = String::from("first");
    map.insert(first, 3);
    println!("map is {:?}", map);
    // error: cannot borrow `map` as mutable because it is also borrowed as immutable
    // println!("first is {:?}", first);

    map.insert(String::from("hello"), 100);
    println!("map is {:?}", map);

    map.entry(String::from("xxx")).or_insert(0);
    println!("map is {:?}", map);

    let p = Point { x: 5, y: 10 };
    println!("p.x is {}", p.x());
    println!("p.y is {}", p.y());
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}