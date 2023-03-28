pub fn print(){
    println!("hello from the other side");

        print::print();
    // basic formatting
    println!("My {} is {} old","Chicken",4);

    // positional arguments
    println!("My {0} is special and {0} can cook a dinner of {1} ","Chicken",6);

    // Named arguments

    println!("My chicken is called {name}, and it likes licking my {name2}",name="Boscow", name2="Other chickens");

    // debug traits

    println!("{:?}",(12,"hi"));
}