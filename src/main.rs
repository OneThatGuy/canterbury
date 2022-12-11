use std::env;
fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("Hello World\n");
    
    let binding = "".to_string();
    let command = args.get(1).unwrap_or(&binding);

    match command.as_str(){
        "fizz" => fizz(),
        "ping" => ping(),
        _ => command_fail(),
    };
    
}



fn command_fail(){
    println!("Unrecognized Command");
}

fn fizz(){
    println!("buzz");
}

fn ping(){
    println!("pong");
}

/*
"setup" => setup()
fn setup_rust(){

}

    take languages the user needs
    for i in langs{
        match i{
            rust => setup_rust()
        }
    }
}


methods maybe
*/