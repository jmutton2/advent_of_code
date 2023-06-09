use std::fs;

fn main() {
    let file_path  = "input.txt";
    // --snip--

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut level = 0;

    let mut in_basement = 0;

    for(index, value) in contents.chars().enumerate() {
        if level < 0 && in_basement == 0 {
            in_basement = index; 
        }
        match value {
            '(' => level += 1,
            ')' => level -= 1,
            _ => (),
        }
    }


    println!("Go to floor: {}. In basement at: {}", level, in_basement);

}
