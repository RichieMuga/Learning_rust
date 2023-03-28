pub fn smile_with_strings(){
    // primitive growable string datastructure
    let mut my_string=String::from("Ohayo Gosaimasu! ");
    println!("{:?}",(my_string));
    // get length
println!("{}",my_string.len());
// push on a char
    my_string.push('1');
// push on a srting
    my_string.push_str("Mavado ni matako");

    // split by whitespace using a for loop
    for word in my_string.split(' '){
        println!("{}",word);
    }

    // test assertion
    assert_eq!(30,my_string.len());

    println!("{}",my_string.capacity());
    
    

}