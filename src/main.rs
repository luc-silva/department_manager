fn main() {
    use Entities::*;
    use Utils::*;

    let mut line = String::new();
    register_company_display(&mut line);

    let comp = Company::new(line);
    show_main_display(&comp);
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
        pub members: Vec<Employee>,
    }
    pub enum Level {
        Senior,
        Mid,
        Junior,
        Intern,
    }
}

pub mod Utils {
    use crate::Entities::Company;

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
}
