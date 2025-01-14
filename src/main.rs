use regex::Regex;

fn operations( expression: &str) -> i32 {
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_sub = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();

    let mut result = 0;
    let mut expression = expression.to_string();

    loop {
        let caps = re_add.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let addition: i32 = left_value + right_value;

        expression = expression.replace(cap_expression, addition.to_string().as_str());
    }

    loop {
        let caps = re_sub.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let subtraction: i32 = left_value - right_value;

        expression = expression.replace(cap_expression, subtraction.to_string().as_str());
    }

    loop {
        let caps = re_mul.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let multiplication: i32 = left_value * right_value;

        expression = expression.replace(cap_expression, multiplication.to_string().as_str());
    }

    loop {
        let caps = re_div.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let division: i32 = left_value / right_value;
        expression = expression.replace(cap_expression, division.to_string().as_str());
    }
    return expression.parse().unwrap();

}

fn main() {
    println!("Hello, world!");

    // Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    // read user data
    println!("Introduce the expresion:");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // apply operations

    let result = operations(expression.as_str());
    
    // print result
    println!("Result: {}", result);
}
