fn main() {
    println!("Hello, world!");

    one_to_ten();
    
    println!("{}", sum_one_to_ten());
}

fn one_to_ten() {
    let mut i : i32 = 0;
    while i<=10 {
        println!("{}", i);
        i+=1;
    }
}

fn sum_one_to_ten() -> i32{
    let mut result = 0;
    let mut i = 1;
    while i<=10{
        result+=i;
        i += 1;
    }
    result
}