pub struct ShellCore {
    pub history: Vec<String>,
}

impl ShellCore {
    pub fn new() -> ShellCore {
        let conf = ShellCore {
            history: Vec::new(),
        };

        conf
    }
}
