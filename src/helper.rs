use std::io::stdin;

pub fn check_is_input_quit(input:&String)->bool{
    return input == "Quit" || input == "quit" || input == "QUIT"
}

pub fn confirm_input()->bool{
    println!("press y to confirm or n to enter again:");
    let mut buf = "".to_string();
    stdin().read_line(&mut buf).expect("unable to read line");
    return buf == "Yes\n" || buf == "yes\n" || buf == "Y\n"|| buf == "y\n";
}

pub fn take_password_input()->String{
    let pass = rpassword::prompt_password("please enter your password:").expect("unable to take pass");
    if confirm_input(){
        return pass;
    }else{
        return take_password_input();
    }
}