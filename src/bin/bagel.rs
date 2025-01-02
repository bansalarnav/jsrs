use std::process;

use bagel::Bagel;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bagel = Bagel::new(args);

    let exit_code = bagel.run();
    process::exit(exit_code);
}
