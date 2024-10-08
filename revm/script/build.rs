use sp1_build::build_program;
use std::env;

fn main() {
    let program_path = if env::var("EOF").unwrap_or_default() == "true" {
        "../program"
    } else {
        "../program_legacy"
    };

    build_program(program_path)
}
