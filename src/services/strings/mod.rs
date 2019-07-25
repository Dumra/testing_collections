use std::io;

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a line");

    string_converter(&input)
}

fn string_converter(input: &String) -> String {
    let mut output = String::new();
    for word in input.split_whitespace() {
        let mut counter = 0;
        let mut end_of_word = String::new();
        for char_in_word in word.to_lowercase().chars().into_iter() {
            if counter == 0 {
                match char_in_word {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'y' => {
                        end_of_word.push_str(format!("-{}ay ", char_in_word).as_str())
                    }
                    _ => end_of_word.push_str("-hay "),
                }
            } else {
                output.push(char_in_word);
            }
            counter += 1;
        }
        output.push_str(end_of_word.as_str());
    }

    output
}
