// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

fn get_user_input() -> Result<String, io::Error> {
    let mut user_input = String::new();

    println!("add {{person}} to {{area}}:");
    io::stdin().read_line(&mut user_input)?;

    Ok(user_input)
}

fn parse_user_input(input: &str) -> (String, String) {
    let split_input = input.split(" to ").collect::<Vec<&str>>();

    let person = split_input[0].replace("add", "").trim().to_string();
    let area = split_input[1].replace(":", "").trim().to_string();

    (area, person)
}

type Positions = HashMap<String, Vec<String>>;

fn add_person(hashmap: &mut Positions, area: String, person: String) {
    match hashmap.get(&area) {
        Some(values) => {
            let mut updated_values = values.clone();
            updated_values.push(person);
            hashmap.insert(area, updated_values);
        }
        None => {
            hashmap.insert(area, vec![person]);
        }
    };
}

fn main() -> Result<(), io::Error> {
    let mut positions: Positions = HashMap::new();
    let mut full_input = vec![];

    loop {
        let user_input = get_user_input()?;

        if user_input.trim() == "end" {
            break;
        };

        let (area, person) = parse_user_input(&user_input);
        full_input.push(person.clone());
        add_person(&mut positions, area, person);
    }

    full_input.sort();
    println!("alphabetzie {:?}", full_input);
    println!("grouped {:?}", positions);

    Ok(())
}
