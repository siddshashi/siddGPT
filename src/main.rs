use sidd_gpt::Data;
use std::process;

const PATH_TO_DATA: &str = "text/tinyshakespeare.txt";

fn main() {
    // TODO: PATH_TO_DATA should be CL arg
    let _data = Data::parse(PATH_TO_DATA).unwrap_or_else(|err| {
        eprintln!("problem parsing data: {err}");
        process::exit(1);
    });
}
