use std::io;
use std::process::exit;

fn main(){
	loop{
		print_start();
		let option = get_number("");
		if option == 1 {
			temp_converter_main();
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

		let i = input.trim_end();

		if i == "y" || i == "Y"{
			exit(0);
		}
		if i == "n" || i == "N"{
			break;
		}
	}
	
}

///Temperature converter
fn temp_converter_main(){
	println!("This program will convert the temperature entered into other temperatures units");
	println!("Enter the unit you want to convert");
	println!("Finish with the unit you want to convert (for ex C is for Celcius)");
	calculate_and_print(read_temp());
}

fn read_temp() -> f32
{
	//TODO: revert the transformation from X unit to Celcius (they are upside down)
	loop{
		let mut line = String::new();
		io::stdin()
		.read_line(&mut line)
		.expect("Failed to read line");
		let mut l = line.trim_end();

		
		//Histogram of temperatures
		//C: Celcius; F: Fahrenheit; K: Kelvins; R: Rankine; De: delisle; N: Newton; Re: Reaumur; Ro: Romer
		if l.ends_with('C'){
			l = l.trim_end_matches('C');
			let n: f32 = match l.trim().parse::<f32>() {
				Ok(num) => num,
				Err(_) => {println!("Please type a valid temperature format!"); continue;},
        			};
			return n;
		} else if l.ends_with('F'){
			l = l.trim_end_matches('F');
			let n: f32 = match l.trim().parse::<f32>() {
				Ok(num) => (num-32.00)/1.8,
				Err(_) => {println!("Please type a valid temperature format!"); continue;},
        			};
			return n;
		} else if l.ends_with('K'){
			l = l.trim_end_matches('K');
			let n: f32 = match l.trim().parse::<f32>() {
				Ok(num) => num-273.15,
				Err(_) => {println!("Please type a valid temperature format!"); continue;},
        			};
			return n;
		}else if l.ends_with('R'){
			l = l.trim_end_matches('R');
			let n: f32 = match l.trim().parse::<f32>() {
				Ok(num) => (num-491.67)/1.8 ,
				Err(_) => {println!("Please type a valid temperature format!"); continue;},
        			};
			return n;
		} else if l.ends_with("De"){
			l = l.trim_end_matches("De");
			let n: f32 = match l.trim().parse::<f32>() {
				Ok(num) => (num- 150.00)/(-1.5) ,
				Err(_) => {println!("Please type a valid temperature format!"); continue;},
        			};
			return n;
		} else if l.ends_with('N') {
			l = l.trim_end_matches('N');
			let n: f32 = match l.trim().parse::<f32>() {
				Ok(num) => num/0.33,
				Err(_) => {println!("Please type a valid temperature format!"); continue;},
        			};
			return n;
		} else if l.ends_with("Re") {
			l = l.trim_end_matches("Re");
			let n: f32 = match l.trim().parse::<f32>() {
				Ok(num) => num/0.8,
				Err(_) => {println!("Please type a valid temperature format!"); continue;},
        			};
			return n;
		} else if l.ends_with("Ro") {
			l = l.trim_end_matches("Ro");
			let n: f32 = match l.trim().parse::<f32>() {
				Ok(num) => (num-7.5)/0.525,
				Err(_) => {println!("Please type a valid temperature format!"); continue;},
        			};
			return n;
		}
		println!("Please make sure that the temperature ends with the valid unit");
		println!("Here's the histogram that this program understands: \n");
		println!("C: Celcius; F: Fahrenheit; K: Kelvins; R: Rankine; De: delisle; N: Newton; Re: Reaumur; Ro: Romer\n");
		println!("Now please enter the temperature again with a valid format: ");
	}
}

fn calculate_and_print(c: f32)
{
	//Histogram of temperatures
	//C: Celcius; F: Fahrenheit; K: Kelvins; R: Rankine; De: delisle; N: Newton; Re: Reaumur; Ro: Romer

	println!("\n");
	println!("Celcius: {}ºC", c);
	println!("Fahrenheit: {}ºF", c*1.8+32.00);
	println!("Kelvins: {}ºK", c+273.15 );
	println!("Rankine: {}ºR", c*1.8+491.67  );
	println!("Delisle: {}ºDe", c*(-1.5)+150.00 );
	println!("Newton: {}ºN", c*0.33 );
	println!("Reaumur: {}ºRe", c*0.8 );
	println!("Romer: {}ºRo", c*0.525+7.5 );
	println!("\n");
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
