fn get_wrapping_paper_amount(l: u32, w: u32, h: u32) -> u32 {
    let slack = l * w;

    let paper_size = 2*l*h + 2*l*w + 2*h*w;

    println!("Input dims: {} {} {} | Paper Size: {}", l, w, h,  paper_size);

    return paper_size + slack;
}

use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_feet = 0;
    
    for line in contents.lines() {
        let line_data: Vec<u32>  = line.split("x").map(|v| v.parse().expect("parse error")).collect();

        total_feet = total_feet + get_wrapping_paper_amount(line_data[0], line_data[1], line_data[2]);

    }

    println!("total_feet: {}", total_feet);

}
