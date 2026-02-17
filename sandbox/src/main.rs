use std::collections::HashMap;
use std::ops::Deref;

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

    let a = "aa";
    let b = "bb";
    println!("longest is {}", longest(a, b));

    use_counter();
    compare_value_with_ref();
    use_my_box();
    test_custom_smart_pointer();
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}

fn longest<'a>(l: &'a str, r: &'a str) -> &'a str {
    if l.len() > r.len() {
        l
    } else {
        r
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6  {
            Some(self.count)
        } else {
            None
        }
    }
}

fn use_counter() {
    let counter = Counter::new();
    for x in counter {
        println!("count = {}", x);
    }
}

fn compare_value_with_ref() {
    let x = 5;
    let y = &x;
    let z = 5;
    println!("x == y({})", 5 == *y);
    println!("x == z({})", x == z);
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn use_my_box() {
    let b = MyBox::new(5);
    println!("{}", *b);

    let b = MyBox::new(String::from("hello"));
    hello_my_box(&b);
}

fn hello_my_box(b: &str) {
    println!("hello {}", b);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn return_smart_pointer() -> CustomSmartPointer {
    CustomSmartPointer { data: String::from("my data") }
}

fn test_custom_smart_pointer() {
    let c = return_smart_pointer();
    std::mem::drop(c);
    println!("end of scope, test_custom_smart_pointer");

    // error: use of moved value: `c`
    // println!("use dropped pointer: {}", c.data)
}