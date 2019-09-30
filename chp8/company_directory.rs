use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
use std::io;

type Department = String;

type Employee = String;

type Listings = HashMap<Department, HashSet<Employee>>;

enum Command {
    ADD,
    LIST,
    HELP,
    COMMANDS,
    EXIT,
    UNKNOWN
}

use Command::*;

const PROGRAM_PROMPT: &str = "$:directory>";

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
            println!("Wrong command format.  Expected `cmd [...args]`\n  (entire call should match `{:?}`).", rx);
            input.clear();
        }
        println!("\n{:}", PROGRAM_PROMPT);
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

fn hashset_to_vec(hs: &HashSet<String>) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    for s in hs.iter() {
        out.push(s.clone());
    }
    return out;
}

fn main() {
    let mut cmds_map: HashMap<String, Command> = HashMap::new();
    cmds_map.insert("add".to_string(), ADD);
    cmds_map.insert("list".to_string(), LIST);
    cmds_map.insert("help".to_string(), HELP);
    cmds_map.insert("commands".to_string(), COMMANDS);
    cmds_map.insert("exit".to_string(), EXIT);

    let cmd_rx: Regex = regex::Regex::new(r"^([a-zA-Z][a-zA-Z_\-\d]{1,55})(\s*[A-Za-z][A-Za-z_\-\d]{1,55})*$").unwrap();

    let mut directory: Directory = Directory::new();
    let mut quit = false;

    loop {
        let args = prompt_for_input_and_extract(&cmd_rx);
        if args.len() == 0 {
            println!("No command given.  Doing nothing.");
            continue;
        }
        let cmd_key = args[0].as_str();
        let cmd = match cmds_map.get(cmd_key) {
            Some(c) => c,
            _ => &UNKNOWN
        };

        match cmd {
            ADD => {
                if args.len() < 3 {
                    println!("Unable to run '{:}' command.\n  \
                        Requires 3 arguments `cmd`, `person`, `department`.\n  \
                        Received {:?} args: {:?}", cmd_key, args.len(), args
                    );
                    continue;
                }
                add_dept_emp(args[2].as_str(), args[1].as_str(), &mut directory);
            }
            // @todo dry up logic here or move commands/actions to separate methods
            LIST => match args.len() {
                2 => {                              // List employees in given dept
                    let dept = &args[1];
                    if !directory.listings.contains_key(dept) {
                        println!("\"{:}\" department not found.", args[1]);
                        continue;
                    }
                    println!("\n\"{:}\" department:", dept);
                    let mut es: Vec<String> = hashset_to_vec(directory.listings.get(dept).unwrap());
                    es.sort();
                    for e in es {
                        println!("- {:}", e);
                    }
                }
                _ => {                              // List all employees by department
                    let mut depts = hashset_to_vec(&directory.departments);
                    if depts.len() == 0 {
                        println!("No entries found.");
                        continue;
                    }
                    depts.sort();
                    for d in depts {
                        println!("\n\"{:}\" department:", d);
                        let mut es: Vec<String> = hashset_to_vec(directory.listings.get(&d).unwrap());
                        es.sort();
                        for e in es {
                            println!("- {:}", e);
                        }
                    }
                }
            }
            HELP => {
                println!("\nAvailable commands:");
                for c in cmds_map.keys() {
                    print!("{:}, ", c);
                }
                println!("\n");
            }
            COMMANDS => {
                println!("\nAvailable commands:");
                for c in cmds_map.keys() {
                    print!("{:}, ", c);
                }
                println!("\n");
            }
            EXIT => {
                quit = true
            }
            UNKNOWN => {
                println!("Command \"{:}\" not found.", cmd_key);
            }
        }
        if quit {
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
