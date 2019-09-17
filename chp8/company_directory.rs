use std::collections::HashMap;

type Store<'a, 'b> = HashMap<&'a str, &'b mut Vec<&'a str>>;

fn add_employee<'a, 'b>(name: &'a str, dept_name: &'a str, store: &'a mut Store<'a, 'b>) {
    add_dept(dept_name, store);
    match store.get(dept_name) {
        Some(xs) => {
            (*xs).push(name);
        }
        _ => ()
    }
}

fn add_dept<'a, 'b>(dept_name: &'a str, store: &'b mut Store<'a, 'b>)  {
    match store.get(dept_name) {
        Some(_) => (),
        _ => {
            let xs: &mut Vec<&str> = &mut vec![];
            store.insert(dept_name, xs);
        }
    }
}

//fn get_dept_directory(name: &str, store: &mut Store) {}
//
//fn get_company_directory() {}

fn main() {}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_add_employee() {}

    #[test]
    fn test_add_dept() {
        let store: &mut Store = &mut HashMap::new();

        add_dept("hello world", store);
        assert_eq!(store.len(), 1 as usize);
    }

    #[test]
    fn test_get_dept_directory() {}

    #[test]
    fn test_get_company_directory() {}
}
