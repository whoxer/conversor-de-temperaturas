# Conversor de temperaturas

Olá, essa é uma atividade do trecho do livro "[Linguagem de programação Rust](https://rust-br.github.io/rust-book-pt-br/title-page.html)", no final do [capítulo 3.5](https://rust-br.github.io/rust-book-pt-br/ch03-05-control-flow.html), o livro te desafia a desenvolver um conversor de temperaturas de Farenheit para Celsius.

Uma tarefa simples se considerar a fórmula: 
```
(°F − 32) × 5 / 9 = °C
```

## Adentrando o código

```rust
use std::io;

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
```

O código começa com o uso da biblioteca padrão ```std::io```, logo após isso a função ```main()``` imprime um texto na tela e declara a variável mutável ```farenheit``` que vai ser lida dentro ```io::stdin()```, a variável de tipo string lida após isso é convertida em uma variável de tipo float de 64 bits.

Aqui em:

```rust
	let celsius = {
		(farenheit - 32.0) * 5.0 / 9.0
	};
```
utilizamos a fórmula aprendida acima e depois imprimimos a conversão seguida do ```break``` que para o ```loop```.