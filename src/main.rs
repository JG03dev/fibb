use std::io;
fn main() {
	starting_msgs();
	let start = get_number("Please enter the start of the sequence");
	let max = get_number("Please enter the number of iterations");
	fibb(start, max, 0, 1);
}

fn starting_msgs(){
	println!("Hello there 👋");
	println!("This program prints the fibonacci sequence with a given start and number of iterations.");
	println!("After some iterations the fibonacci sequence goes very high!");
	println!("This program reaches up to iteration 185. Which is 205697230343233228174223751303346572685");
	print!("To start ");
}

fn get_number(msg: &str)-> u128{
	let mut _num = 0;
	loop{
		let mut input = String::new();
		println!("{msg}");
		io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

		_num = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => {println!("Error thats not a number! Consider using just positive numbers"); continue},
		};
		break;
	}
	_num
}

fn fibb(start: u128, mut max: u128, num: u128, next: u128){
	if max > 0 && num <= (u128::MAX - next)
	{
		//print once we start
		if start <= num {
			println!("{num} ");
			//print!("{max} ");
			max = max - 1;
		}
		//recursive call
		fibb(start, max, next, num+next);
	}
}