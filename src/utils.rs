use crate::entities::*;

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