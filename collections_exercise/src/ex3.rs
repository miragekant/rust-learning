use std::collections::HashMap;

pub fn show_employees(company: &HashMap<String, Vec<String>>) {
    for (department, _) in company {
        println!("Department: {}", department);
        show_employees_by_department(company, department);
    }
}

pub fn show_employees_by_department(company: &HashMap<String, Vec<String>>, department: &str) {
    let employees = company.get(&(department.to_string())).unwrap();
    for e in employees {
        println!("{}", e);
    }
} 

pub fn add_employee_to_department(company: &mut HashMap<String, Vec<String>>, department: &str, employee: &str) {
    let d = company.entry(department.to_string()).or_insert(Vec::new());
    d.push(employee.to_string());
} 

