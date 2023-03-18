fn main(){
    //Main function

    //Creates variables for user inputs and translated sentance
    let mut text = String::new();
    let mut sent = String::new();
    //Establishes vectors containing vowels
    //Second vector is created to allow for y to be treated as a vowel mid sentance
    let vowels = ['a','e','i','o','u'];
    let vowels_y = ['a','e','i','o','u','y'];
    //Introduction and user inputs
    println!("Welcome to the pig latin translator!");
    println!("Please enter a sentance to be translated: ");
    //Takes user inputs. Code found at https://www.tutorialspoint.com/rust/rust_input_output.htm
    let _b1 = std::io::stdin().read_line(&mut text).unwrap();
    //Converst sentance into vector based on whitespace
    //Iterates thorugh each word and translates individually
    for x in text.split_whitespace(){
        //Boolean variable used to check if word has been translated or not
        let mut t_check = false;
        //Converts word to only alphabet. Removes special letters
        let mut clean_text = get_alphabets(x.to_string());
        //Puts word in lowercase
        clean_text = clean_text.to_lowercase();
        //Uses logic to skip empty words
        if clean_text.is_empty(){
            continue;
        } else{
            //Checks to see if the word is shorter than two letters
            if clean_text.len() <= 2{
                //If the word is shorter than two letters, doesn't rearrange letters
                //Adds "way" to end of word and adds new word to translated sentance
                let short_word = clean_text + "way" + " ";
                sent.push_str(&short_word);
                //Continues loop
                continue;
            } else{
                //Variable that is assigned to the first letter of the word.
                //Found by turning word into characters and going to first letter.
                let first_char = clean_text.chars().next().unwrap();
                //Checks to see if the first word is a vowel or not
                if vowels.contains(&first_char){
                    //If first letter is vowel, "yay" is added and word is added to end of new sentance
                    let vowel_word = clean_text + "yay" + " ";
                    sent.push_str(&vowel_word);
                    continue;
                } else{
                    //Creates variable that represents letters attached to the end of the word before the "ay"
                    let mut backend = String::new();
                    //Itterates through each letter of the new word
                    for y in clean_text.chars(){
                        //Checks to see if the character is in the list of vowels that include y
                        if vowels_y.contains(&y){
                            //If y is in our list, we use all the letters that were placed in the backend and remove them from the front with a replacen function
                            //We add the backend to the end of our word with an "ay"
                            let normal_word = clean_text.replacen(&backend, "" ,1) + &backend + "ay" + " ";
                            //New word is added to the end of our sentance
                            sent.push_str(&normal_word);
                            //Verifies that the word has been translated
                            t_check = true;
                            //Ends loop
                            break;
                        } else{
                            //If the letter isn't a vowel, it's added to the backend
                            let z = y.to_string();
                            backend.push_str(&z);
                        }
                    }
                    //If the word didn't meet any of the prior requirements, "ay" is just added to the end
                    if t_check == false{
                        let bad_word = clean_text + "ay" + " ";
                        sent.push_str(&bad_word);
                    }
                }
            }
        }
    }
    //Capitalizes the first letter
    sent = uppercase_first(&sent);
    println!("");
    //Prints original sentance and translated sentance
    println!("Original Sentance: {}", text);
    println!("Translated Sentance: {}", sent);
}

/// Found online at https://docs.rs/qndr/latest/src/qndr/lib.rs.html#46-56
pub fn get_alphabets(sample:String )->String{
    let mut alphabets = String::from(""); 
    for c in sample.chars() { 
        let r = c.is_alphabetic();
        match r {
            true=> {alphabets.push(c)},
            false=> {},
        }
    }
    alphabets    
}

// Found online at https://www.dotnetperls.com/uppercase-first-letter-rust
fn uppercase_first(data: &str) -> String {
    // Uppercase first letter.
    let mut result = String::new();
    let mut first = true;
    for value in data.chars() {
        if first {
            result.push(value.to_ascii_uppercase());
            first = false;
        } else {
            result.push(value);
        }
    }
    result
}