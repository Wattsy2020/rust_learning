pub fn translate(text: &str) -> String {
    let translated_words: Vec<String> = text
        .split(' ')
        .map(translate_word)
        .collect();
    translated_words.join(" ")
}

fn translate_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(char) => if is_vowel(&char) {
            format!("{word}-hay")
        } else {
            format!("{}-{}ay", chars.as_str(), char)
        }
    }
}

/// Check if an english character is a vowel
fn is_vowel(char: &char) -> bool {
    match char {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_with_consonant_word() {
        assert_eq!(translate("Hello"), "ello-Hay");
        assert_eq!(translate("there"), "here-tay");
    }

    #[test]
    fn test_translate_with_vowel_word() {
        assert_eq!(translate("alliteration"), "alliteration-hay");
        assert_eq!(translate("ourselves"), "ourselves-hay");
    }
}