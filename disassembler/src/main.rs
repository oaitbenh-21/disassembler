mod module;
use module::record;
// use shared::file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for argfile_path in &args[1..] {
        // match file::validate_core_file(arg) {
        // Ok(file_path) => {
        // start working on that and turn binary to assembly code.
        record::BinaryRecord::run_disassembler(argfile_path.clone());
        // }
        // Err(e) => {
        // println!("{}", e);
        // }
    }
}
// let arr: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
// read_magic_code(&arr);
// }
