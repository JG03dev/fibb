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
	let n = get_number("Enter the position you want to get");
	println!("The fibbonacci number in this position is: {}\n", fibb(0, n, 0, 1));
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

fn fibb(start: u128, mut max: u128, num: u128, next: u128)-> u128{
	if max > 0 && num <= (u128::MAX - next)
	{
		max = max - 1;
		//recursive call
		return fibb(start, max, next, num+next);
	}
	return num;
}

///Print Song
fn print_song(){

}
