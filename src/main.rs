// Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in
// a department or all people in the company by department,
// sorted alphabetically

//Scenario:
//TODO: Add check for department number in list of departments
//TODO: Print whole company
//TODO: "Department added" printout should be shown with department name instead of number
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

    let mut departments_users: HashMap<String, u8> = HashMap::new();

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
         
        //check that exists
        // if !departments.contains(added_department) {
        //     println!("Non-existing department", );
        // }
        


        departments_users.insert(u.first_name.to_string(), added_department);

        println!("{} added to {}.", u.first_name, added_department);
        println!("");
    }
    println!("{:?}", departments_users);
}

#[derive(Debug)]
struct Department {
    dep_id: u8,
    dep_name: String,
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

// fn add_user_to_department(user: User) -> bool {
//     true
// }
