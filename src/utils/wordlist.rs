use std::fs;

pub struct Wordlist;

impl Wordlist {
    pub fn range(from: u32, to: u32) -> Vec<String> {
        let mut wordlists: Vec<String> = Vec::new();

        for i in from..to + 1 {
            wordlists.push(i.to_string());
        }

        wordlists
    }

    pub fn file(path: String) -> Vec<String> {
        let read_string = fs::read_to_string(path).expect("Failed to read wordlists file");
        let content: Vec<&str> = read_string.split("\n").collect();

        content.iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wordlist_file() {
        let r = Wordlist::file("./Cargo.toml".to_string());
        let content = r.join("\n");
        let cargo_toml = fs::read_to_string("./Cargo.toml").expect("Failed to read wordlists file");
        assert_eq!(content, cargo_toml);
    }

    #[test]
    fn wordlist_range() {
        let wordlist = Wordlist::range(0, 20);
        assert_eq!(
            wordlist,
            vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15", "16", "17", "18", "19", "20"
            ]
        )
    }
}
