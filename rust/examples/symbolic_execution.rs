use std::{
    collections::HashMap,
    io::{self, stdout, BufRead, Write},
    process::exit,
    sync::Mutex,
};

use libcomplex_polynomials::polynomial::Polynomial;
use once_cell::sync::Lazy;

static MAP: Lazy<Mutex<HashMap<String, Polynomial>>> = Lazy::new(|| Mutex::new(HashMap::new()));

const USER_INPUT_PROMPT: &str = "User Input:      ";
const TERMINAL_OUTPUT_PROMPT: &str = "Terminal Output: ";

fn main() {
    repl()
}

fn repl() {
    println!("To use this REPL, you can use two types of statements:");
    println!("- Polynomials definitions, which are of the following form:");
    println!("      A = (1.23 + 3.45i)X2 + (-2.0 - 1.0i)X + (-1.0 + 0.0i)");
    println!("      B = (1 - 2i)X3 + (-1 + 0i)");
    println!("- Expressions, which lets you add/substract/multiply polynomials.");
    println!("  There are no parenthesis and members are evaluated from left to right:");
    println!("      A + B");
    println!("      A * B + B");
    println!("");
    println!("You can type `exit` to exit.");
    println!("---------");

    print_flush(USER_INPUT_PROMPT);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("couldn't read line");
        process_input(&line);
        print_flush(USER_INPUT_PROMPT);
    }
}

fn print_flush(str: &str) {
    print!("{}", str);
    stdout().flush().expect("couldn't flush stdout");
}

fn process_input(line: &str) {
    if line == "exit" {
        exit(0);
    } else if line.contains('=') {
        // Handle polynomial definition
        let mut split = line.split('=');
        let name = split
            .next()
            .expect("no polynomial identifier before '='")
            .trim()
            .to_string();
        let definition: Polynomial = split
            .next()
            .expect("no polynomial definition after '='")
            .into();

        MAP.lock().unwrap().insert(
            name.trim().to_string(),
            definition
                .clone()
                .try_into()
                .expect("couldn't parse polynomial"),
        );

        println!(
            "{}Registered polynomial {name} with definition {definition}",
            TERMINAL_OUTPUT_PROMPT
        );
    } else {
        // Handle expression
        print_flush(TERMINAL_OUTPUT_PROMPT);

        let line = line.trim();
        let mut operations = vec!["+"];
        let mut polynomials = vec![];
        for token in line.split(' ') {
            match token {
                t @ "+" | t @ "-" | t @ "*" => {
                    operations.push(t);
                }
                _ => {
                    let map = MAP.lock().unwrap();
                    if let Some(poly) = map.get(token) {
                        polynomials.push(poly.to_owned());
                    } else {
                        println!("unknown polynomial: {}", token);
                        return;
                    }
                }
            }
        }

        let mut value = Polynomial::zero();
        let mut i = 0;

        for &operation in operations.iter() {
            match operation {
                "+" => {
                    let rhs = &polynomials[i];
                    value = Polynomial::add(&value, &rhs);
                }
                "-" => {
                    let rhs = &polynomials[i];
                    let rhs = Polynomial::neg(&rhs);

                    value = Polynomial::add(&value, &rhs);
                }
                "*" => {
                    let rhs = &polynomials[i];
                    value = Polynomial::mul(&value, &rhs);
                }
                _ => {
                    println!("unknown operation {}", operation);
                    return;
                }
            }

            i += 1;
        }

        println!("{value}");
    }
}
