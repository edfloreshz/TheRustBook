use std::collections::HashMap;

struct Directory {
    directory: HashMap<String, String>
}

impl Directory {
    pub fn add_employee(&mut self, command: &str) {
        let command_destructured: Vec<&str> = command.split(' ').collect();
        match self.directory
            .insert(
                command_destructured[1].to_string(),
                command_destructured[3].to_string()
            ) {
            Some(_) => {},
            None => {}
        }
        ()
    }
    fn print_directory(&mut self) {
        println!("Employee Directory\n");
        for (employee, department) in self.directory.iter() {
            println!("{} is in the {} department", employee, department)
        }
    }
}

pub fn test_employee() {
    let mut directory = Directory {directory: HashMap::new()};
    directory.add_employee("Add Sarah to Sales");
    directory.add_employee("Add James to Sales");
    directory.add_employee("Add Louis to Commerce");
    directory.add_employee("Add Sophia to Development");
    directory.add_employee("Add Eddy to Development");
    directory.print_directory();
}

