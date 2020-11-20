//  Created by Eduardo Flores on 5/18/20.
//  edfloreshz@gmail.com

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and
// “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to
// the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

pub fn test_transform() {
    println!("{}", transform("first apple hello there ß∂©∑´".to_string()));
}

pub fn transform(text: String) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut words: Vec<String> = words.iter().map(|word| word.to_string()).collect();
    for word in &mut words {
        match word.chars().next() {
            Some(letter) => {
                match letter {
                    'a' | 'e' | 'i' | 'o' | 'u' => {
                        word.push_str("-hay");
                    }
                    'b'..='z' | 'B'..='Z' => {
                        word.remove(0);
                        word.push_str(format!("-{}ay", letter).as_str());
                    }
                    _ => { }
                }
            }
            None => continue
        }
    }
    words.join(" ")
}
