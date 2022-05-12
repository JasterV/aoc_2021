use super::DigitDisplay;

pub struct NoteEntry {
    pub input: Vec<DigitDisplay>,
    pub output: Vec<DigitDisplay>,
}

impl From<String> for NoteEntry {
    fn from(entry: String) -> Self {
        let split_digits = |str: &str| -> Vec<DigitDisplay> {
            str.trim()
                .split_whitespace()
                .map(DigitDisplay::from)
                .collect()
        };
        let parts: Vec<&str> = entry.split(" | ").collect();
        let first = parts.first().expect("An entry requires an input part.");
        let last = parts.last().expect("An entry requires an output part");
        let input = split_digits(first);
        let output = split_digits(last);
        Self { input, output }
    }
}
