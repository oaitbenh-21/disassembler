mod module;
use module::record;
use shared::file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for argfile_path in &args[1..] {
        match file::validate_core_file(argfile_path) {
            Ok(file_path) => {
                // start working on that and turn binary to assembly code.
                let result = record::BinaryRecord::run_disassembler(file_path.clone());
                println!("{}", result);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
