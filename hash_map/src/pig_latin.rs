//  Created by Eduardo Flores on 5/18/20.
//  edfloreshz@gmail.com

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and
// “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to
// the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

use std::ops::Index;

enum LetterType {
    Consonant(char, char, char, char, char),
    Vowel
}

pub fn transform(text: String) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut words: Vec<String> = words.iter().map(|word| word.to_string()).collect();
    for word in &mut words {
        while let Some(c) = word.chars() {
            let suffix = match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    word.push(c);
                    String::from("-hay")
                }
                'a'...'z' | 'A'...'Z' => {
                    format!("-{}ay", c)
                }
                _ => {
                    word.push(c);
                    continue;
                }
            };
            word.push_str(suffix.as_str());
        }
    }

    words.join(" ")
}