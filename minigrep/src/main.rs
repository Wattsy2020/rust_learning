mod argparse;

fn main() {
    let arguments = argparse::read_arguments();
    println!("Searching {} for pattern: {}", arguments.filename(), arguments.pattern());
}
