mod entities;
mod utils;

use entities::*;
use utils::*;

fn main() {
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
