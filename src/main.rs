fn main() {
    println!("Hello, world!");
    let ans = is_even(401);
    println!("{}" , ans);
    // println!(ans); error
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0{
        return true;
    }else {
        return false;
    }

}