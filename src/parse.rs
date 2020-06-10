// Imports
use csv::Reader;
use std::error::Error;
use csv::StringRecord;

// Rule
pub struct Rule {
    pub from: String,
    pub to: Vec<String>
}

// Takes StringRecord output from raw CSV values and coverts it to type `Rule`
pub fn parse_helper(rule: StringRecord) -> Rule {
    if let Some(host) = rule.get(0) { 
        if let Some(tos) = rule.get(1) { 
            let to_arr: Vec<String> = tos.split("|").map(|s| s.to_string()).collect();
            return Rule { from: host.to_string(), to: to_arr }
        }
    }
    // Error handling. Will cause intentional program crash
    Rule { from: "Error".to_string(), to: vec!["Error".to_string()]}
}
pub fn parse() -> Result<Vec<Rule>, Box<dyn Error>> {
    let mut rules : Vec<Rule> = Vec::new();
    let mut rdr = Reader::from_path("src/data.csv")?;
    for result in rdr.records() {
        // `?` indicates that if result returns an error, the error is passed to the return
        // statement as type `Result`
        let rule = result?;
        rules.push(parse_helper(rule));
    }
    // Returns `Rule` vector as type `Result`
    Ok(rules)
}
