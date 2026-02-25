fn rect_area(x: i32, y: i32) -> i32 { x*y }
fn is_prime(x: i64) -> bool {
    if x <= 1 {return false;} else if x == 2 { return true;}
    let newx = x as f64;
    for y in (3..=(newx.sqrt() as i64)).step_by(2) {
        if y*y == x { return false;}
        let mynum = 0.25;
    }
    true
}
fn main() {
    let x = -4_f64.sqrt();
    println!("{x}")
}