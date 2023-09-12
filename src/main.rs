use std::os;

fn main() {
    use Entities::*;
    use Utils::*;

    let mut line = String::new();
    register_company_display(&mut line);

    let mut comp = Company::new(line);
    show_main_display(&comp);

    show_options();
    let mut option = String::new();
    std::io::stdin().read_line(&mut option);

    option = extract_first_char(option);

    if option == "0" {
        comp.employees.push(create_employee());
    } else if option == "1" {
        insert_employee_display(&comp);
    } else if option == "2" {
        comp.departments.push(create_department());
    } else {
        std::process::exit(1);
    }
}

pub mod Entities {
    pub struct Company {
        pub name: String,
        pub departments: Vec<Department>,
        pub employees: Vec<Employee>,
    }
    impl Company {
        pub fn get_employees_total(&self) -> i32 {
            let mut total = 0;
            for department in &self.departments {
                total += department.members.len();
            }
            return total as i32;
        }

        pub fn get_department_total(&self) -> i32 {
            return self.departments.len() as i32;
        }

        pub fn list_departments(&self) {
            let department_length = &self.departments.len();

            if *department_length == 0 {
                println!("Theres no departments. Create one!");
                return;
            }

            let mut counter: i32 = 0;
            for department in &self.departments {
                println!("[{}] - {}", counter, department.name);
                counter += 1;
            }
        }

        pub fn list_employees(&self) {
            let employees_length = &self.departments.len();

            if *employees_length == 0 {
                println!("Theres no employees. Register one!");
                return;
            }

            let mut counter: i32 = 0;
            for employee in &self.employees {
                println!("[{}] - {}", counter, employee.name);
                counter += 1;
            }
        }

        pub fn new(comp_name: String) -> Company {
            return Self {
                name: comp_name,
                departments: vec![],
                employees: vec![],
            };
        }
    }
    pub struct Employee {
        pub name: String,
        pub age: i32,
        pub salary: i32,
        pub level: Level,
    }
    pub struct Department {
        pub name: String,
        pub members: Vec<String>,
    }
    impl Department {
        pub fn list_members(&self) {
            let mut counter: i32 = 0;
            for member in &self.members {
                println!("[{}] - {}", counter, member);
            }
        }
    }

    pub enum Level {
        Senior,
        Mid,
        Junior,
        Intern,
    }
}

pub mod Utils {
    use crate::Entities::{Company, Department, Employee, Level};

    pub fn show_main_display(comp: &Company) {
        println!("========= Department Manager =========");
        println!("Company name: {}", comp.name);
        println!("Departments : {}", comp.get_employees_total());
        println!("Employees   : {}", comp.get_department_total());
        println!("======================================");
    }

    pub fn register_company_display(line: &mut String) {
        println!("===== Department Manager Creator =====");
        println!("Company name: ");
        std::io::stdin().read_line(line).expect("fail");
        println!("--------------------------------------");
    }

    pub fn show_options() {
        println!("Which operation do you want to do?");
        println!("[0] Register employee");
        println!("[1] Add employee to a department");
        println!("[2] Register department");
        println!("[*] Exit");
        println!("--------------------------------------");
    }

    pub fn create_department() -> Department {
        let mut name = String::new();
        let mut members: Vec<String> = vec![];

        println!("Enter name:");
        std::io::stdin().read_line(&mut name);

        return Department { name, members };
    }

    pub fn insert_employee_display(comp: &Company) {
        let department_size = &comp.departments.len();
        let employees_size = &comp.employees.len();

        if *department_size == 0 || *employees_size == 0 {
            println!("Returning to main.");
            return;
        }
        
        let mut department_option_input = String::new();
        let mut employee_option_input = String::new();

        let mut department_option = 0;
        let mut employee_option = 0;

        println!("Which employee do you want to add to a department?");
        println!("Employees:");
        comp.list_employees();
        std::io::stdin()
            .read_line(&mut employee_option_input)
            .expect("Fail");

        employee_option = employee_option_input.parse().expect("Not a integer");

        println!("Chose a department");
        println!("Departments:");
        comp.list_departments();

        std::io::stdin().read_line(&mut department_option_input).expect("fail");
        department_option = department_option_input.parse().expect("Not a integer");

        let mut choose_employee = comp.employees.get(employee_option);
        

        let mut choose_department = comp.departments.get(department_option);
        match choose_department {
            Some(department) => {
                match choose_employee {
                    Some(employee) => {
                        department.members.push(employee.name.clone())
                    }, None => (),
                }
            }, None => (),
        }

    }

    pub fn create_employee() -> Employee {
        let mut age_input = String::new();
        let mut age: i32 = 0;

        let mut salary_input = String::new();
        let mut salary = 0;

        let mut level: Level = Level::Junior;
        let mut name = String::new();

        println!("Enter name:");
        std::io::stdin().read_line(&mut name);

        println!("Enter salary:");
        std::io::stdin().read_line(&mut salary_input);
        salary = salary_input.parse().expect("Not a interger");

        println!("Enter age:");
        std::io::stdin().read_line(&mut age_input);
        age = age_input.parse().expect("Not a interger");

        return Employee {
            age,
            name,
            salary,
            level,
        };
    }

    pub fn extract_first_char(str: String) -> String {
        let mut single_char = String::new();

        for char in str.chars() {
            single_char.push(char);
            break;
        }

        return single_char;
    }
}
