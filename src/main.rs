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

}
