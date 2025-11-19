fn main() {

    let b:(i32, bool, f64) = (110, true, 10.9);
    print(b);
}

fn print(x:(i32, bool, f64)) {
    println!("Inside print method");
    //assigns a tuple to distinct variables
    let (age,is_male,cgpa) = x;
    println!("Age is {}, is Male? {}, cgpa is {}",age,is_male,cgpa);
}
