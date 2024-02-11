use std::env;
use std::fs;

/*
TODO:
    Find magic numbers in files
 */

/*
NOTES
Declare variable types with ':', like ':f32' or ':i8' or ':char'
Macros are referenced with '!', like 'println!'
Variables are immutable, apply 'mut' to change em
*/

fn main() {
    // collects arguments given into a vector (array)
    let args: Vec<String> = env::args().collect();

    // String is dynamically allocated; str is fixed. '&' is the reference operator (references value, not object itself)
    let cmd_args = parse_args(&args);

    // Rust expects Result enum to have error handling; make sure it does.
    let contents = fs::read(&cmd_args.file_path)
        .expect("Couldn't find file");
    // No real error handling right now

    println!("{:?}", contents);

    // Get first four bytes of file
    let first_bytes: &[u8] = &contents[0..4];

    // iter: for each slice of first_bytes do what's inside brackets
    for byte in first_bytes.iter(){
        println!("0x{:x}", byte);
    }
    print_file_info(&cmd_args.file_path);
}

// All of this is from following the guide on modularity from the rust docs
struct CmdArguments {
    file_path: String
}

// -> operator is for return types (a bit confusing)
fn parse_args(args: &[String]) -> CmdArguments{
    let file_path;
    // clone method is similar to '&', but just clones the value of an object
    file_path = args[1].clone();
    CmdArguments{ file_path }
}

fn print_file_info(file_path: &String) {
    let metadata = fs::metadata(file_path)
        .expect("Couldn't find file");
    println!("{:?}", metadata.file_type());
    println!("File is: {} bytes long", metadata.len());
}