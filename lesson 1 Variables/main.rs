enum Family{
    Father,
    Mom,
    Son,
    Daughter
}
struct Person{
    name: Family,
    age: i32
}
fn print_person( person: Person ){
    match person.name{
        Family::Father => println!("Father's name is Kim"),
        Family::Mom => println!("Mom's name is Oko"),
        Family::Son => println!("Son's name is Dni"),
        Family::Daughter => println!("Daughter's name is Hannah"),
    }
    println!("age: {:?}", person.age);
}
fn main(){
    let father = Person {
        name: Family::Father,
        age: 50
    };
    let mom = Person {
        name: Family::Mom,
        age: 42
    };
    print_person(mom);
    print_person(father);
}
