mod entry;
mod password_manager;
mod helper;
use password_manager::PasswordManager;

fn main(){
    if let Some(mut pm) = PasswordManager::new("PMfiles/safe.pswd".to_string()){
        pm.menu();
    }else{
        println!("could not open a safe file in the directory: PMfiles/safe.pswd");
    }

}