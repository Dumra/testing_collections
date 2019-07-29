use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;

const DEPARTMENTS: &[&str] = &["Engineering", "Sales", "Marketing", "Analytics"];

pub fn run() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    show_employee_list();
    loop {
        println!("If you want to add employee please type `A` or type `E` to exit or 'C' to continue:");
        let input_value = readline();
        match input_value {
            Ok(input) => {
                let input = input.to_lowercase();
                let words: Vec<&str> = input.split_whitespace().collect();
                match words.len() {
                    1 => {
                        if *words.get(0).unwrap() == 'a'.to_string() {
                            unimplemented!();
                        } else if *words.get(0).unwrap() == 'e'.to_string() {
                            break;
                        } else {
                            continue;
                        }
                    }
                    _ => {
                        println!("Incorrect input. Try again!");
                        continue;
                    }
                }
            }
            Err(error) => panic!("Problem parse input: {:?}", error),
        }
        break;
    }
}

fn show_employee_list() {
    println!("Here's an employee list:");
    println!("{:#?}", DEPARTMENTS);
    println!("Pattern for adding employee to the list is: Add <Name> to <Department>");
}

fn readline() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}

fn some(company: &mut  HashMap<String, Vec<String>>, employee: String)-> Result<bool, io::Error> {
    let employee = employee.to_lowercase();
    let words: Vec<&str> = employee.split_whitespace().collect();
    if words.len() == 4 {
        let department = words.get(3).unwrap();
        if company.contains_key(*department) {
            let employees = company.get_mut(*department).unwrap();
            employees.push(words.get(1).unwrap().to_string());
        } else {
            company.insert(
                department.to_string(),
                vec![words.get(1).unwrap().to_string()],
            );
        }
        Ok(true);
    } else {
        io::Error::new(ErrorKind::InvalidInput)
    }
}
