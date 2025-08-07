pub fn recite(mut start_bottles: u32, take_down: u32) -> String {
    let number_in_words = ["no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
    
    let mut result = String::new();

    let mut count = 0;
    loop {
        let bottle_or_bottles = if start_bottles == 1 { 
            format!("{} green bottle", number_in_words[start_bottles as usize]) 
        } else { 
            format!("{} green bottles", number_in_words[start_bottles as usize]) 
        };

        result.push_str(&format!(
            "{bottle_or_bottles} hanging on the wall,\n",
        ));
        result.push_str(&format!(
            "{bottle_or_bottles} hanging on the wall,\n",
        ));
        result.push_str("And if one green bottle should accidentally fall,\n");

        start_bottles = start_bottles.saturating_sub(1);
        
        if start_bottles == 0 {
            result.push_str("There'll be no green bottles hanging on the wall.\n\n");
        } else {
            let next_bottle_or_bottles = if start_bottles == 1 { 
                format!("{} green bottle", number_in_words[start_bottles as usize]).to_lowercase() 
            } else { 
                format!("{} green bottles", number_in_words[start_bottles as usize]).to_lowercase()
            };
            result.push_str(&format!(
                "There'll be {next_bottle_or_bottles} hanging on the wall.\n\n",
            ));
        }

        count += 1;
        if count == take_down {
            break
        }
    }
      
    result
}
