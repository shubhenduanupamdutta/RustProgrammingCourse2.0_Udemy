//----------------------------------------------------------------
//        Regular Expression Basics
//----------------------------------------------------------------
use regex::Regex;

pub fn main() {
    let re = Regex::new(r"[prt]ain").unwrap();

    let text = "rrain spain none";
    println!("The text: {:?}\nhas a match: {:?}", text, re.is_match(text));

    println!();
    println!("Finding all matches in the text:");
    for mat in re.find_iter(text) {
        println!("Found: {:?}", mat.as_str());
    }

    println!("Finding first match in the text:");
    if let Some(mat) = re.find(text) {
        println!("Found: {:?}", mat.as_str());
    }

    println!();
    println!("Finding all matches in the text using captures:");
    for cap in re.captures_iter(text) {
        println!("Found: {:?}", &cap.get(0).unwrap().as_str());
    }

    println!();
    println!("Using dots to match any character:");
    let re = Regex::new(r"[prt].ain").unwrap();
    let text = "rrain spain ptain";
    for capture in re.captures_iter(text) {
        println!("Found: {:?}", &capture[0]);
    }

    println!();
    println!("Matching gray or grey:");
    let re = Regex::new(r"gr[ae]y").unwrap();
    let text = "gray grey gruy graye";
    for capture in re.captures_iter(text) {
        println!("Found: {:?}", &capture[0]);
    }

    println!();
    println!("Matching a range of characters:");
    let re = Regex::new(r"[a-z]ain").unwrap();
    let text = "main pain tain rain but not Pain Rain Tain Main";
    for capture in re.captures_iter(text) {
        println!("Found: {:?}", &capture[0]);
    }

    println!();
    println!("Excluding a range of characters using ^:");
    let re = Regex::new(r"[^a-z]ain").unwrap();
    for capture in re.captures_iter(text) {
        println!("Found: {:?}", &capture[0]);
    }

    println!();
    println!("Using a shorthand for a range of characters:");
    let re = Regex::new(r"\d\d\d\d\d\d").unwrap();
    let text = "My phone number is 816030 and the second phone number is 123456";
    for capture in re.captures_iter(text) {
        println!("Found: {:?}", &capture[0]);
    }

    println!();
    println!("Matching with start of the text using ^:");
    let re = Regex::new(r"^aba").unwrap();
    let text = "abacus abalone aback aba abaa bc";

    for capture in re.captures_iter(text) {
        println!("Found: {:?}", &capture[0]);
    }


    println!();
    println!("Matching with end of the text using $:");
    let re = Regex::new(r"bc$").unwrap();
    let text = "aba abc bc";
    for capture in re.captures_iter(text) {
        println!("Found: {:?}", &capture[0]);
    }

    println!();
    println!("Matching with start and end of the text using ^ and $:");
    let re = Regex::new(r"^bc$").unwrap();
    let text = "bc";
    for capture in re.captures_iter(text) {
        println!("Found: {:?}", &capture[0]);
    }

    let text2 = "b adf lkj c";
    for capture in re.captures_iter(text2) {
        println!("Found: {:?}", &capture[0]);
    }

    println!();
    println!("Matching with word boundaries using \\b:");
    let re = Regex::new(r"\b\w").unwrap();
    let text = "Hi my name is Shubhendu";
    for capture in re.captures_iter(text) {
        println!("Found: {:?}", &capture[0]);
    }

    println!();
    println!("Using `*` to match zero or more occurrences:");
    let re = Regex::new(r"\b\w*").unwrap();
    for capture in re.captures_iter(text) {
        println!("Found: {:?}", &capture[0]);
    }
}