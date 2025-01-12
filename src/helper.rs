use std::io::{stdin, stdout, Write};

pub fn check_is_input_quit(input:&String)->bool{
    return input == "Quit" || input == "quit" || input == "QUIT"
}

pub fn take_input(prompt:&String, is_pass:bool)->Option<String>{
    let mut buf = "".to_string();
    if is_pass{
        buf = rpassword::prompt_password(prompt).expect("Unable to get password from user");
        if check_is_input_quit(&buf){
            return  None;
        }
        return Some(buf);
    }else{
        print!("{}",prompt.clone()+" ");
        stdout().flush().expect("unable to flush lines to terminal");
        stdin().read_line(&mut buf).expect("unable to read line");
        if check_is_input_quit(&buf){
            return  None;
        }
        return Some(buf[..buf.len()-1].to_string());
    }
    
}

pub fn take_confirmed_input(prompt:String, is_pass:bool)->String{
    loop{
        // keep taking input while the user does not confirm
        let input = take_input(&prompt, is_pass);
        if input.is_some(){
            return input.unwrap();
        }
        let confirmation = take_input(&"Confirm input? y/n or q to quit".to_string(), false)
            .unwrap_or("n".to_string()).to_lowercase();
        if confirmation == "y"{
            return  input.unwrap();
        }if confirmation == "q"{
            return  "".to_string();
        }
    }
}

pub fn take_password_input()->String{
    take_confirmed_input("Please enter your password:".to_string(), true)
}

