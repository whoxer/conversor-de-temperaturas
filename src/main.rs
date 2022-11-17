use std::io;

// formula:
// (quantidade°F − 32) × 5/9 = quantidade°C

fn main() {
	println!("\t<conversor de temperatura>");
	let mut farenheit = String::new();
				
	loop {
		io::stdin()
		   .read_line(&mut farenheit)	
		   .expect("falhou em ler a linha");
				
		let farenheit: f64 = match farenheit.trim().parse() {
			Ok(num) => num,
			Err(_)  => continue,
		};
		
		let celsius = {
			(farenheit - 32.0) * 5.0 / 9.0
		};
		
		println!("{farenheit}°F é igual a {celsius}°C");
		break;
	}
}
