// Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in
// a department or all people in the company by department,
// sorted alphabetically

//Scenario:
//TODO: Interface for adding users manually
//TODO: Interface for adding manually, from list, print (e.g q=quit)
//TODO: Redo if department doesn't exist
//TODO: Print whole company
//TODO: Print alphabetically (by lastname), by department. As function
//TODO: "Add user" as function

#![allow(unused_variables)]
// #[warn(dead_code)]
#[allow(dead_code)]

use std::io;
use std::collections::HashMap;
fn main() {
    let departments = initialize_departments();
    let users = initialize_users();

    //let mut departments_users: HashMap<String, u8> = HashMap::new();
    let mut departments_users: HashMap<String, &String> = HashMap::new();
    

    println!(
        "There are {} new users in system, add these to correct department:",
        users.len()
    );
    println!("",);

    for u in &users {
        for d in &departments {
            println!(
                "Type {} to add {} to {}",
                d.dep_id, u.first_name, d.dep_name
            );
        }

        let mut input_string = String::new();

        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        let added_department: u8 = input_string.trim().parse::<u8>().expect("Wanted a number");

        //check that input exists
        // if !departments.iter().any(|department: &Department| department.dep_id == added_department) {  //alternative to below
        // if !departments.iter().any(|d: &Department| d.dep_id == added_department) {  //alternative //alternative to below
        if !departments.iter().any(|ref d| d.dep_id == added_department) {
            println!("Non-existing department, user not added. Please add a valid department");
        }

        let department_name: &String = &departments
            .iter()
            .find(|d| d.dep_id == added_department)
            .expect("")
            .dep_name;

        departments_users.insert(u.first_name.to_string(), department_name);
        //departments_users.insert(u.first_name.to_string(), added_department);
        println!("{} added to {}.", u.first_name, department_name);
        println!("");
    }

    println!("Type c to print all users in company, sorted alphabetically",);
    println!("Type d to print all users in department, sortet alphabetically",);
    println!("Type cd to print all users in company, by department, sorted alphabetically",);

    let mut print_operation = String::new();

    io::stdin()
        .read_line(&mut print_operation)
        .expect("Failed to read line");

    //print_string(print_operation);
    let mut print_company: Vec<_> = departments_users.iter().collect();

    match print_operation.trim().as_ref() {
        //"c" => println!("All users in company printed {:?}", print_company),
        "c" => {
            println!("All users in company:", );
            print_company.sort_by(|a, b| a.cmp(b));
            print_company.iter().for_each(|&x| println!("{}", x.0))
        }
        "d" => println!("All users by department printed"),
        "cd" => {
            println!("All users in company, by department, sorted alphabetically: ");
            print_company.sort_by(|a, b| a.1.cmp(b.1).then_with(|| a.cmp(b)));
            print_company.iter().for_each(|&x| println!("{}, {}", x.0, x.1))
        },
        _ => (println!("LOL")),
    }
} //main

#[derive(Debug)]
struct Department {
    dep_id: u8,
    dep_name: String,
}

impl PartialEq for Department {
    fn eq(&self, other: &Department) -> bool {
        self.dep_id == other.dep_id
    }
}

#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    e_mail: String,
}

fn initialize_users() -> Vec<User> {
    let corp_mail_extension = "@coolcorp.com";
    let mut users: Vec<User> = Vec::new();

    let user1 = User {
        first_name: String::from("John"),
        last_name: String::from("McEnroe"),
        e_mail: String::from("jm@coolcorp.com"),
    };
    users.push(user1);

    let user2 = User {
        first_name: String::from("Ivan"),
        last_name: String::from("Lendl"),
        e_mail: String::from("il@coolcorp.com"),
    };
    users.push(user2);

    users.push(User {
        first_name: String::from("Andre"),
        last_name: String::from("Agassi"),
        e_mail: String::from("aa@coolcorp.com"),
    });
    users.push(User {
        first_name: String::from("Boris"),
        last_name: String::from("Becker"),
        e_mail: format!("bb{}", corp_mail_extension),
    });
    users
}

fn initialize_departments() -> Vec<Department> {
    let mut departments: Vec<Department> = Vec::new();

    departments.push(Department {
        dep_id: 1,
        dep_name: String::from("Sales"),
    });
    departments.push(Department {
        dep_id: 2,
        dep_name: String::from("HR"),
    });
    departments.push(Department {
        dep_id: 3,
        dep_name: String::from("Engineering"),
    });
    departments.push(Department {
        dep_id: 4,
        dep_name: String::from("IT"),
    });

    departments
}
