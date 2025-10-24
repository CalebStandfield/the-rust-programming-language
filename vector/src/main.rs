fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];

    let mut v = Vec::new();

    // .push to add values to a vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];


    // let does_not_exist = &v[100]; // Panics if ran
    // _let does_not_exist = v.get(100);

    let out_of_bounds_error: Option<&i32> = v.get(100);
    match out_of_bounds_error {
        Some(out_of_bounds_error) => println!("The code somehow found a value out of bounds! {out_of_bounds_error}"),
        None => println!("No value since its out of bounds. Thus we get a value of None"),
    }

    // Iteration
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}