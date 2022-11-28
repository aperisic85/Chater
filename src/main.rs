fn main() {
    let mut test = String::from("hello");

    println!("length of {} is {} ", test, calculate_string_size(&test));
    println!(" print this as slice: {}",&test[0..2]);

    fn calculate_string_size (s:&String) -> usize {
        s.len()
    }

    fn check_space_no (s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate(){
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }
}
