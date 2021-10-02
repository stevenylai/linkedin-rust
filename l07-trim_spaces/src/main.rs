fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(to_trim: &str) -> &str {
    let mut first_untrim = false;
    let mut last_char: char = ' ';
    let mut start: usize = 0;
    let mut end: usize = 0;
    for (idx, c) in to_trim.char_indices() {
        if c != ' ' {
            if idx == 0 {
                first_untrim = true;
            }
            if !first_untrim && start == 0 {
                start = idx;
            }
        }
        if last_char != ' ' {
            end = idx;
        }
        last_char = c;
        //println!("xx{} {} {} {}", idx, c, start, end);
    }
    if last_char != ' ' {
        end = to_trim.len();
    }
    return &to_trim[start .. end];
}
/* YOUR CODE GOES HERE */