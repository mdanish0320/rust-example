fn main() {
    println!("Hello, world!");

    // mutability()
    // tuple_example()
    // array_example()
    // vector_example()
    // array vs vector vs slice, slice is the reference of array
    // tuple vs tuple-structs
    // hash_map_example()
    vector_of_struct_example()
}

fn mutability() {
    
    /** mutation not allowed **/
    let x = 1;

    // x = 6; // not allowed, produce error
    println!("The value of x is: {}", x);

    let x = x + 1; // allowed
    println!("The value of x is: {}", x);


    
    /** mutation allowed **/
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6; // allowed
    println!("The value of x is: {}", x);
    

    
}

fn tuple_example() {
    let tup = (500, 6.4, 1);
    println!("The value of y is: {}", tup.0);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}

fn array_example() {
    // By default, arrays are immutable.
    // fixed in size, memory:stack
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);

    ///////////// example 2
    // let a = [1, 2, '3', 4, '5']; // not allowed
    
}

fn array_loop_example() {
    let a = [1, 2, 3];

    println!("a has {} elements", a.len());
    for e in a.iter() {
        println!("{}", e);
    }
}

fn vector_example() {
    // flexible size, memory:heap
    let mut nums = vec![1, 2, 3]; // mut nums: Vec<i32>

    nums.push(4);

    println!("The length of nums is now {}", nums.len()); // Prints 4

    for e in nums.iter() {
        println!("{}", e); // Prints 1, 2, 3
    }
}




struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn struct_example() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

}

fn hash_map_example(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    // insert
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // access
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


}

fn vector_of_struct_example(){
    // definition
    #[derive(Debug)]
    struct St1 {
            name: String,
            age: u32,
    }

    // initialization
    let rect1 = St1 {
        name: "danish".to_owned(),
        age: 50,
    };

    let mut v = vec![];
    v.push(rect1);

    println!("{:?}", v);

    // loop-1
    for st in v.iter() {
         println!("{:?}", st.name);
    }

    // loop-2
    for (pos, e) in v.iter().enumerate() {
        println!("{}: {:?}", pos, e);
    }

    // loop-3
    for i in v {
        // iterate by-value
        println!("{:?}", i);
    }

    
}

    
