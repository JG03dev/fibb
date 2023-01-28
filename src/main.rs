use std::io;
use std::process::exit;

fn main(){
	loop{
		print_start();
		let option = get_number("");
		if option == 1 {
			f_to_c_main();
		} else if  option == 2 {
			fibb_main();
			ask_exit();
		} else if option == 3 {
			print_song();
		}
		ask_exit();
	}

}

fn print_start(){
	print!("{}[2J", 27 as char); //clear (major part of) the scren
	println!("Hello world!");
	println!("This is an Open Source Project I use to upload my exercises while learning the Rust Programming Language.");
	println!("To start choose one of these different games");
	println!("
	1. Convert temperatures between Fahrenheit and Celsius. \n
	2. Generate the nth Fibonacci number. \n
	3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song. \n
	- Any other number else to exit.
      ");
	print!("Your choice: ");	
}

fn ask_exit(){
	println!("Do you want to exit? Y/N");
	loop{
		let mut input = String::new();
		io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");

		input.pop(); input.pop();//eliminate whitespace

		if input == "y" || input == "Y"{
			exit(0);
		}
		if input == "n" || input == "N"{
			break;
		}
	}
	
}

///F to C
fn f_to_c_main(){

}


///Fibonacci
fn fibb_main() {
	starting_msgs();
	let start = get_number("Please enter the start of the sequence");
	let max = get_number("Please enter the number of iterations");
	fibb(start, max, 0, 1);
}

fn starting_msgs(){
	//println!("Hello there 👋");
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

///Print Song
fn print_song(){

}
