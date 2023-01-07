fn main() {
	
	let mut x =5;
	let mut y;
	let mut z;

	println!("x is {}", x);
	
	y = 2.1;
	z = -1;

	println!("y is {}",y);
	println!("z is {}",z);



	let spaces = "   ";
	let spaces = spaces.len();

	println!("The value of spaces is {}",spaces);



    let tup:(u16, f64, i8) = (500, -3.3, -3);

    (x,y,z) = tup;

    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
    println!("The value of z is : {}", z);

	x = 3;

	println!("{}",x);


	let arr1:[i16;2] = [500, 455];

	let arr_r:[f64;2] = [3.4, 1.1];

	let arr = [0;3];

	println!("{}",arr1[0]);
	println!("{}",arr_r[1]);
	println!("{}",arr[0]);

}
