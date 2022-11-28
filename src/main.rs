use std::collections::HashMap;

fn main() {
    let mut test = String::from("hello");
    println!(" print this as slice: {}",&test[0..2]);

    let names = vec!["ANTE", "MATE", "IKO", "NIKO", "KATA", "KARLA"];
    println!("names before map() {:?}", names);

    let low_names: Vec<String> = names.iter()
        .map(|word| word.replace("TE","BLEEE"))
        .map(|word| word.to_lowercase())
        .collect();
    println!("name after map{:?}", low_names);

    let text:String= String::from("croatia canada 4 - 0. World Cup");
    println!("Text: {}", text);
    let letters = "abc";

   let result: HashMap<_,_> = letters.chars()
       .map(|a| (a, text.matches(a).count()))
       .collect();
    println!(" count letters : {:?}", result)
}
