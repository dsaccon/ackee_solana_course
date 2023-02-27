use clap::Parser;

fn main() {
    let args = Cli::parse();
    let result = match args.operator.as_str() {
        "add" => add(&args.arg_1, &args.arg_2), 
        "sub" => sub(&args.arg_1, &args.arg_2), 
        "mult" => mult(&args.arg_1, &args.arg_2), 
        "div" => div(&args.arg_1, &args.arg_2),
        _ => std::f32::INFINITY,
    };
    if result.is_infinite() {
        println!("Incorrect operator: '{}', must be 'add', 'sub', 'mult' or 'div'", args.operator);
    } else {
        println!("Result: {}", result);
    }
}

fn add(a1: &f32, a2: &f32) -> f32 {
    let sum = (a1 + a2).into();
    sum
}

fn sub(a1: &f32, a2: &f32) -> f32 {
    let diff = (a1 - a2).into();
    diff
}

fn mult(a1: &f32, a2: &f32) -> f32 {
    let prod = (a1 * a2).into();
    prod
}

fn div(a1: &f32, a2: &f32) -> f32 {
    let quo: f32 = a1 / a2;
    quo
}

#[derive(Parser)]
struct Cli {
    arg_1: f32,
    arg_2: f32,
    operator: String,
}
