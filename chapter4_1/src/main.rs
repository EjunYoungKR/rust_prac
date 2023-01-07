use std::io;

fn main() {
	println!("[1] Convert Fahrenheit to Celsius");
	println!("[2] Convert Celsius to Fahrenheit");

	let choice = loop {
		println!("Input the choice");

		let mut choice = String::new();

		io::stdin().read_line(&mut choice)
			.expect("Fail to read line");

		let choice :u32 = match choice.trim().parse(){
			Ok(num) => num,
			Err(_) => {
				println!("Input number 1 or 2");
				continue
			},
		};

		if choice > 0 && choice < 3 {
			break choice;
		}
	};

	let tem = loop {
		println!("Input Temperature");

		let mut tem = String::new();

		io::stdin().read_line(&mut tem).expect("Fail to read line");

		let tem :f64 = match tem.trim().parse(){
			Ok(num) => num,
			Err(_) => {
				println!("You input wrong temperature!");
				continue
			},
		};

		break tem;
	};

	if choice == 1 {
		let result = f_to_c(tem);
		println!("{}C",result);
	} else {
		let result = c_to_f(tem);
		println!("{}F",result);
	};


}

fn f_to_c(x:f64) ->f64 {
	(x -32.0)/1.8
}

fn c_to_f(x:f64) ->f64 {
	x*1.8 + 32.0
}

