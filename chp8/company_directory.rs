use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
use std::io;

type Department<'a> = &'a str;

type Employee<'a> = &'a str;

type Listings<'a> = HashMap<Department<'a>, HashSet<Employee<'a>>>;

fn prompt_for_input(rx: &Regex) -> String {
    let mut input = String::new();
    let mut ind = 0;
    while !rx.is_match(input.trim()) {
        if ind > 0 { // if asked for input and regex match failed ...
            println!("Wrong command format.  Should be `cmd [...args]`.");
            input.clear();
        }
        println!("\n$:directory>");
        match io::stdin().read_line(&mut input) {
            Err(e) => {
                println!("Command error: {:?}", e);
            }
            Ok(_) => ()
        }
        ind += 1;
    }
    return input;
}

fn parse_input<'a, 'b>(rx: &'a Regex, input: &'b str) -> Vec<&'b str> {
    let mut args: Vec<&'b str> = vec![];
    let cs = rx.captures(input.trim()).unwrap();
    let parts = cs.get(0).unwrap().as_str().split(" ");
    for p in parts {
        let s: &'b str = p;
        args.push(s);
    }
    args
}

fn main<'a, 'b>() {
    let cmd_rx: Regex = regex::Regex::new(r"^(add|exit)(\s*[A-Za-z_]{2,55})+$").unwrap();
    let mut listings: Listings = HashMap::new();
    let mut quit = false;
    loop {
        let input: String = prompt_for_input(&cmd_rx);
        let args = parse_input(&cmd_rx, &input);
        if args.len() == 0 {
            println!("No command given.  Doing nothing.");
            continue;
        }
        match args[0] {
            "add" => {
                if args.len() >= 3 {
                    println!("Unable to run 'add' command.  Need more args.  Received: {:?}", args);
                } else {
                    add_dept_emp(args[1], args[0], &mut listings)
                }
            }
            "exit" => {
                quit = true
            }
            _ => {
                println!("No command found.  Doing nothing");
            }
        }
        if quit {
            break;
        }
    }
}

fn add_dept_emp<'a, 'b>(dept_name: &'a str, emp_name: &'a str, hash_map: &'b mut Listings<'a>) {
    hash_map.entry(dept_name).or_insert(HashSet::new())
        .insert(emp_name)
    ;
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_add_dept_emp() {
        let depts: Vec<Department> = vec!["business", "development", "marketing"];
        let emps: Vec<Employee> = vec!["Emp 1", "Emp 2", "Emp 3"];
        let mut _listings: Listings = HashMap::new();
        let mut i: usize = 0;
        let mut j: usize = 0;
        for d in depts {
            while j <= i {
                println!("\nadd_dept_emp({:?}, {:?}, {:?})", d, emps[j], _listings);
                add_dept_emp(d, emps[j], &mut _listings);
// @todo count the individual entries in each hash_set
                j += 1;
            }
            assert_eq!(_listings.len(), i + 1);
            j = 0;
            i += 1;
        }
    }
}