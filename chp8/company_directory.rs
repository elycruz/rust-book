use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
use std::io;

type Department = String;

type Employee = String;

type Listings = HashMap<Department, HashSet<Employee>>;

const PROMPT: &str = "$:directory>";

#[derive(Debug)]
struct Directory {
    listings: Listings,
    departments: HashSet<Department>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            listings: HashMap::new(),
            departments: HashSet::new(),
        }
    }
}

fn prompt_for_input_and_extract(rx: &Regex) -> Vec<String> {
    let mut input = String::new();
    let mut ind = 0;
    while !rx.is_match(input.trim()) {
        if ind > 0 { // if asked for input and regex match failed ...
            println!("Wrong command format.  Should be `cmd [...args]`.");
            input.clear();
        }
        println!("\n{:}", PROMPT);
        match io::stdin().read_line(&mut input) {
            Err(e) => {
                println!("Command error: {:?}", e);
            }
            Ok(_) => ()
        }
        ind += 1;
    }
    let mut args: Vec<String> = vec![];
    let cs = rx.captures(input.trim()).unwrap();
    let parts = cs.get(0).unwrap().as_str().split(" ");
    for p in parts {
        args.push(String::from(p));
    }
    args
}

fn main() {
    let cmd_rx: Regex = regex::Regex::new(r"^(add|exit|list)(\s*[A-Za-z_]{2,55})+$").unwrap();
    let mut directory: Directory = Directory::new();
    let mut quit = false;
    loop {
        let args = prompt_for_input_and_extract(&cmd_rx);
        if args.len() == 0 {
            println!("No command given.  Doing nothing.");
            continue;
        }
        match args[0].as_str() {
            "add" => {
                if args.len() < 3 {
                    println!("Unable to run 'add' command.  Need more args.  Received: {:?}", args);
                } else {
                    add_dept_emp(args[2].as_str(), args[1].as_str(), &mut directory);
                    println!("{:?}", directory);
                }
            }
            "list" => {
                match args.len() {
                    2 => {
                        let dept = &args[1];
                        if directory.listings.contains_key(dept) {
                            // @todo list people for given department (alphabetically)
                            println!("Employees in \"{:}\" department:", dept);
                            let mut es: Vec<String> = vec![];
                            let emps = directory.listings.get(dept).unwrap();
                            for e in emps {
                                es.push(e.clone());
                            }
                            es.sort();
                            for e in es {
                                println!("- {:}", e);
                            }
                        } else {
                            println!("\"{:}\" department not found.", args[1]);
                        }
                    }
                    _ => {
                        // @todo list all persons in all depts alphabetically
                    }
                }
            }
            "exit" => {
                quit = true
            }
            _ => {
                println!("No command found.  Doing nothing");
            }
        }
        if
        quit {
            break;
        }
    }
}

fn add_dept_emp<'a, 'b>(dept_name: &'a str, emp_name: &'a str, dir: &'b mut Directory) {
    dir.listings.entry(dept_name.to_string()).or_insert(HashSet::new())
        .insert(emp_name.to_string());
    dir.departments.insert(dept_name.to_string());
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_add_dept_emp() {
        let depts: Vec<&str> = vec!["business", "development", "marketing"];
        let emps: Vec<&str> = vec!["Emp 1", "Emp 2", "Emp 3"];
        let mut _directory: Directory = Directory::new();
        let mut i: usize = 0;
        let mut j: usize = 0;
        for d in depts {
            while j <= i {
                println!("\nadd_dept_emp({:?}, {:?}, {:?})", d, emps[j], _directory);
                add_dept_emp(d, emps[j], &mut _directory);
// @todo count the individual entries in each hash_set
                j += 1;
            }
            assert_eq!(_directory.listings.len(), i + 1);
            j = 0;
            i += 1;
        }
    }
}
