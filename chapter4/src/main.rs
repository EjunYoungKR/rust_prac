use std::io;


fn main() {
    let mut number = String::new();

	io::stdin()
		.read_line(&mut number)
		.expect("Fail to read line");

	let number:u32 = number.trim().parse()
		.expect("Please type a number");

	println!("you enter {}",number);

	if number % 4 == 0 {
		println!("number is divisible by 4");
	} else if number % 3 == 0 {
		println!("number is divisible by 3");
	} else if number % 2 == 0 {
		println!("number is divisible by 2");
	} else {
		println!("number is NOT divisible by 4, 3, or 2");
	}

}
