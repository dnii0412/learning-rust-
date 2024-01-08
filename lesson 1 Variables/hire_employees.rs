enum Employee{
    One,
    Two,
    Three,
    Four,
    Five
}
struct Person{
    name: Employee,
    age: i32,
    years_of_exp: i32,
    projects: i32,
    married: bool
}
fn print_who ( person: &Person ){
    match person.name{
        Employee::One => println!("This employee's name is: Bob "),
        Employee::Two => println!("This employee's name is: Sally "),
        Employee::Three => println!("This employee's name is: Allen "),
        Employee::Four => println!("This employee's name is: Brim "),
        Employee::Five => println!("This employee's name is: Jack ")
    }
    println!("This employee age is {}", person.age);
    println!("This employee years of experience {}", person.years_of_exp);
    println!("This employee has worked on these numbers of projects {}", person.projects);
    println!("This employee has been married? {}",  person.married);
}
fn main(){
    let people = vec![
        Person{ name:Employee::One, age: 32, years_of_exp: 10, projects: 38, married: true },
        Person{ name:Employee::Two, age: 22, years_of_exp: 4, projects: 16, married: false },
        Person{ name:Employee::Three, age: 28, years_of_exp: 7, projects: 85, married: false },
        Person{ name:Employee::Four, age: 48, years_of_exp: 28, projects: 384 , married: true },
        Person{ name:Employee::Five, age: 37, years_of_exp: 19, projects: 231, married: true }
    ];
    for Person in &people{
        if Person.age < 40 && Person.years_of_exp > 15 && Person.projects > 50 && Person.married == true{
            println!("Congrats!!! This employee is passed");
            print_who(Person);
        }
    };
}
