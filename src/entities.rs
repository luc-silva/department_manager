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
