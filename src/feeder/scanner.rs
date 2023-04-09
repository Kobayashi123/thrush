use super::Feeder;

impl Feeder {
    pub fn scanner_word(&mut self) -> usize {
        let mut ans = 0;
        for char in self.remaining.chars() {
            if let Some(_) = " \t\n;&|".find(char) {
                break;
            }
            ans += char.len_utf8();
        }
        ans
    }

    pub fn scanner_blank(&mut self) -> usize {
        let mut ans = 0;
        for char in self.remaining.chars() {
            if let Some(_) = " \t".find(char) {
                ans += 1;
            } else {
                break;
            }
        }
        ans
    }

    pub fn scanner_job_end(&mut self) -> usize {
        if let Some(char) = self.remaining.chars().nth(0) {
            if let Some(_) = ";&\n".find(char) {
                return 1;
            }
        }
        0
    }
}
