pub struct Bagel {
    args: Vec<String>,
}

impl Bagel {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }

    pub fn run(self) -> i32 {
        let result = match self.args.len() {
            1 => Self::run_prompt(),
            2 => Self::run_file(),
            _ => {
                println!("Usage: bagel [source]")
            }
        };

        0
    }

    fn run_prompt() {}
    fn run_file() {}
}
