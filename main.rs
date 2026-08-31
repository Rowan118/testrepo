use std::io;

fn main() {
    listen_to_input();
}

fn listen_to_input(){

    loop{
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input){
            Ok(_input_size) => {
                // println!("{}", user_input);
                match user_input.as_str().trim(){
                    "exit\n" => {break;}
                    _ => {parse_command_line(&user_input);}
                }
            },
            Err(e) => {
                println!("Failed to get user input: {:?}", e);
            }
        }
    } 
}

fn parse_command_line(input_cmd: &str){
    // Turns out for splitting like how a shell needs it, we need to implement this ourselves
    let parsed_cmd = input_cmd.split_whitespace();

    for current_command in parsed_cmd{
        println!("{}", current_command);
    }
}
