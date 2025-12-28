mod module;
use module::record;
use shared::file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for argfile_path in &args[1..] {
        if *argfile_path == "--hint".to_string() {
            println!(
                "Example of humans using the Linux shell:\n\t\t./disassembler file.cor > file.asm"
            );
            return;
        }
        match file::validate_core_file(argfile_path) {
            Ok(_) => {
                let result = record::BinaryRecord::run_disassembler(argfile_path.clone());
                println!("{}", result);
                return;
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }
    println!("Usage:\n\t\t./disassembler [filePath.cor]");
    println!("\t\tcargo run [filePath.cor]");
    println!("Example:\n\t\t./disassembler ../players/bin/player.cor");
    println!(
        "Help:\n\t\tIf you need to write that in a file, learn how humans use the Linux shell\n\t\tor use --hint to help you."
    );
}
