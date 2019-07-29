use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;

const DEPARTMENTS: &[&str] = &["Engineering", "Sales", "Marketing", "Analytics"];

pub fn run() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    show_employee_list();
    loop {
        println!(
            "If you want to add employee please type `A` or type `E` to exit or 'C' to continue:"
        );
        let input_value = readline();
        match input_value {
            Ok(input) => {
                let input = input.to_lowercase();
                let words: Vec<&str> = input.split_whitespace().collect();
                match words.len() {
                    1 => {
                        if *words.get(0).unwrap() == 'a'.to_string() {
                            println!("Input ur employee!");
                            match add_employee(
                                &mut company,
                                match readline() {
                                    Ok(value) => value,
                                    Err(e) => {
                                        println!("Some error occurred: {}", e);
                                        continue;
                                    }
                                },
                            ) {
                                Ok(_) => {
                                    println!("Company structure: {:#?}", company);
                                    continue;
                                }
                                Err(_) => {
                                    println!("Incorrect string. Try again");
                                    continue;
                                }
                            }
                        } else if *words.get(0).unwrap() == 'e'.to_string() {
                            break;
                        } else {
                            println!("Input Department name or Full to show the list of employee");
                            let default = "full".to_string();
                            show_list(&readline().unwrap_or(default), &company);
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

fn add_employee(
    company: &mut HashMap<String, Vec<String>>,
    employee: String,
) -> Result<bool, io::Error> {
    let employee = employee.to_lowercase();
    let words: Vec<&str> = employee.split_whitespace().collect();
    if words.len() == 4 {
        let department = words.get(3).unwrap().to_lowercase();
        let departments: Vec<String> = DEPARTMENTS
            .to_vec()
            .iter()
            .map(|x| x.to_lowercase())
            .collect();
        if departments.contains(&department) {
            if company.contains_key(&*department) {
                let employees = company.get_mut(&*department).unwrap();
                employees.push(words.get(1).unwrap().to_string());
            } else {
                company.insert(
                    department.to_string(),
                    vec![words.get(1).unwrap().to_string()],
                );
            }
            return Ok(true);
        }
    }
    Err(io::Error::new(ErrorKind::InvalidInput, "Incorrect input!"))
}

fn show_list(input: &str, company: &HashMap<String, Vec<String>>) {
    let departments: Vec<String> = DEPARTMENTS
        .to_vec()
        .iter()
        .map(|x| x.to_lowercase())
        .collect();
    let input = input.trim().to_lowercase();
    if departments.contains(&input) {
        if company.contains_key(&input) {
            let mut employees = company.get(&*input).unwrap().clone();
            employees.sort();
            println!("List: {:#?}", employees);
        } else {
            println!("No one employee in {} department", input);
        }
    } else {
        let mut employees_list: Vec<String> = vec![];
        for employees in company.values() {
            employees_list.extend(employees.clone());
        }
        employees_list.sort();
        println!("List of campany's employees: {:#?}", employees_list);
    }
}
