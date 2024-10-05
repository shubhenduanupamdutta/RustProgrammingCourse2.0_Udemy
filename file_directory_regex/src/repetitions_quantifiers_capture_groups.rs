//----------------------------------------------------------------
//      Regex - Repetitions, Quantifiers, Capture Groups
//----------------------------------------------------------------

use regex::Regex;

pub fn main() {
    let re = Regex::new(r"a?aa").unwrap();
    let text1 = "aa aaa";
    for cap in re.captures_iter(text1) {
        println!("Found match: {}", &cap[0]);
    }

    println!();
    println!("For the regex pattern 'ba?' and text 'a ba b ba':");
    let re = Regex::new(r"ba?").unwrap();
    let text = "a ba b ba";
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }

    println!();
    println!(r"Use of quantifier to get file of particular type. For the regex pattern '\w?\w?\w?.rs' and text 'file1.rs file2.txt file3.rs':");
    let re = Regex::new(r"\w?\w?\w?.rs").unwrap();
    let text = "fil.rs t1.rs file.rs";
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }

    println!();
    println!("Use of '+' quantifier.");

    println!("For the regex pattern 'a+' and text 'a aa aaa baab bab':");
    let re = Regex::new(r"a+").unwrap();
    let text = "a aa aaa baab bab";
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }

    println!();
    println!("Looking for files with '.gif' extension, which have at least one character before '.gif'.");
    let re = Regex::new(r"\w+\.gif").unwrap();
    let text = "image1.gif and background.gif";

    println!(r"Matching pattern: '\w+\.gif' with text 'image1.gif and background.gif'");
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }

    println!();
    println!("Use of '*' quantifier.");
    let re = Regex::new(r"ab*").unwrap();
    let text = "a ab abbbbb";
    println!("For the regex pattern 'ab*' and text 'a ab abbbbb':");
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }

    println!();
    println!(r"Use of '{{}}' for specific number of repetitions.");
    limited_occurrence();

    println!();
    println!("Capture groups.");
    capture_groups();
}

fn limited_occurrence() {
    let re = Regex::new(r"\b\w{3,5}\b").unwrap();
    let text = "Hello I think you are because I have a gift for you.";
    println!(r"For the regex pattern '\b\w{{3,5}}\b' and text 'Hello I think you are because I have a gift for you.'");
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }

    println!();
    println!("Using '{{n,}}' for minimum number of repetitions.");
    let re = Regex::new(r"\b\w{3,}\b").unwrap();
    let text = "Hello I think you are because I have a gift for you.";
    println!(r"For the regex pattern '\b\w{{3,}}\b' and text 'Hello I think you are because I have a gift for you.'");
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }

    println!();
    println!("Using '{{,n}}' for maximum number of repetitions.");
    let re = Regex::new(r"\b\w?{5}\b").unwrap();
    let text = "Hello I think you are because I have a gift for you.";
    println!(r"For the regex pattern '\b\w{{,5}}\b' and text 'Hello I think you are because I have a gift for you.'");
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }
    

    println!();
    println!("Making sure that the number of digits in the whole and fraction parts are between 1 to 3 digits with a dot in between.");
    let re = Regex::new(r"\b\d{1,3}\.\d{1,3}\b").unwrap();
    let text = "921.583 0.0 1456.25";
    println!(r"For the regex pattern '\b\d{{1,3}}\.\d{{1,3}}\b' and text '921.583 0.0 1456.25'");
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }
}

fn capture_groups() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2021-07-23 2021-08-01 2021-09-30";
    println!(r"For the regex pattern '(\d{{4}})-(\d{{2}})-(\d{{2}})' and text '2021-07-23 2021-08-01 2021-09-30'");
    for cap in re.captures_iter(text) {
        println!("Month: {}, Day: {}, Year: {}, and whole match: {}", &cap[2], &cap[3], &cap[1], &cap[0]);
    }
}