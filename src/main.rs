mod automate;
use automate::Automate;

fn main() {
    let auto =
        Automate::from_file(include_str!("../automate.txt")).expect("can't parse the automate");
    println!("Debug of the automate:{:?}", &auto);
    let test = "abbbbaabbaaabbbaabb";
    println!(
        "the string : {} acceptetion: {}",
        test,
        auto.is_word_accepted(test)
    )
}
