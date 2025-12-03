struct Employee {
    ceo: String,
    company: String,
    age: u32,
}

fn main() {
    let emp1 = Employee {
        company: String::from("microsoft corporation"),
        ceo: String::from("sayta nadella"),
        age: 56,
    };

    let emp2 = Employee {
        company: String::from("google inc"),
        ceo: String::from("sundar pichai"),
        age: 51,
    };

    display(&emp1);  // borrow emp1
    display(&emp1);  // can borrow again
}

fn display(emp: &Employee) {  // receive a reference
    println!("name is: {} company is: {} age is: {}", emp.ceo, emp.company, emp.age);
}