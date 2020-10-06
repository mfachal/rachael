use std::io;
use std::io::Write;

static OPS: [&str; 13] = ["ADD", "SUB", "MUL", "DIV", "AND", "OR", "NOT", "EQ", "DIFF", "LT", "LTE", "GT", "GTE"];

fn main() {
    let mut stack: Vec<i32> = Vec::new();       // Global stack
    let mut code: Vec<String> = Vec::new();     // Reusable codes array
    let pos: usize = 0;                     // Current code pos.
    // TODO: hashmap to store variables         // Global variables

    loop {
        
        print!("stackmachinima> ");
        io::stdout().flush().expect("Err: Unexpected error");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Err: Line read failed");

        // println!("{}", line);

        // Build and clean code vector
        // TODO >>>

        // Evaluate each code position
        // TODO: iterar code array e evaluar cada una.
        // Si alguna da error capaz cortar o seguir con
        // la siguiente
        match evaluate(&mut stack, &mut code, pos) {
            Ok(()) => println!("Ok..."),
            Err(err) => println!("Err: {}", err)
        }

        // Print stack and env
        // TODO >>>
    }
}

// Esto llena code array con cada operación/literal y también limpia la línea
// Ej. remueve espacios adicionales y esas cosas
fn parse_line(line: String, code: &mut Vec<String>, pos: &mut usize) -> Result<(), String> {
	let mut iter = line.split_whitespace();
	loop {
		match iter.next() {
			Some(s) => code.push(s.to_string()),
			None => break
		}
	}
    return Ok(());
}

fn build_code(code: &mut Vec<String>) -> Result<(), String>{

	for mut op in code.iter_mut() {
		op.make_ascii_uppercase();
		if OPS.contains(&op.as_str()) {continue;}
		if op.starts_with("GET:") {continue;}
		if op.starts_with("SET:") {continue;}
		if op.parse::<i64>().is_ok() {continue;};
		return Err("Invalid program!".to_string());
	}
	return Ok(());
}


// Evalúa una posición de code array y actualiza el stack
// Acá tenemos que usar alguna otra función que determine el tipo de operación
// Recibe también la pos de code mutable, así podemos hacer por ej. operaciones de JUMP en code
fn evaluate(stack: &mut Vec<i32>, code: &mut Vec<String>, mut pos: usize) -> Result<(), String> {

	while pos < usize::MAX {


		if code[pos] == "ADD".to_string() {/*stack.push(stack.pop() + stack.pop());*/} //convertir a i32 porque esto va a romper


		pos += 1;
		if pos >= code.len() {pos = usize::MAX;}
	}



    // return Err("pepe".to_string());
    return Ok(());
}
