fn is_armstrong_number(num: i32) -> bool {
    let len = ((num + 1) as f32).log10().ceil() as i32;
    let strigified = num.to_string();
    let armstrong: i32 = strigified.chars()
        .map(|digit| digit.to_digit(10).unwrap())
        .map(|digit| digit.pow(len))
        .sum();
    armstrong == num
}
fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let num = get_input().trim().parse::<i32>().unwrap();
    if is_armstrong_number(num) {
        println!("armstrong number");
    } else {
        println!("not armstrong number");
    }
}
