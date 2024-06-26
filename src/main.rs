mod entry;
mod password_manager;
mod helper;
use std::{clone, env, fs::{self, File}, io::{stdin, Read, Write}, path::Path, process::exit, vec};
use std::fs::read_to_string;
use aead::{OsRng, KeyInit, Aead, AeadCore, generic_array::GenericArray, Key};
use aes_gcm::{Aes256Gcm, AesGcm};
use base64::{engine::general_purpose, Engine};
// use password_manager::PasswordManager;
use pbkdf2::pbkdf2_hmac;
use rand::{RngCore, Rng};
use sha2::Sha256;

/*
    pbkdf usage: 
        -   use pbkdf to generate a key based on a password called a "key encryption key" or KEK
            use this "KEK" to encrypt the key used to encrypt/decrypt the data (actual key)
        -   store only the salt and encrypted actual key
        -   to get from KEK to actual key -> do pbkdf2 on a given password and the salt, then try to decrypt the encrypted using the result 
            you may store then nonce to assist in the deryption of the actual key as the nonce does not matter
            if the KEK was correct then it will decrypt successfully, otherwise will return error
            if done successfully store it in the option key and init cipher based on that key
*/

use entry::Entry;

use crate::helper::take_input;
#[allow(unused,dead_code)]
struct PM{
    filepath: String,
    entries: Vec<Entry>,
    cipher: Option<Aes256Gcm>
}
#[allow(dead_code,unused)]

#[allow(unused,dead_code)]
impl PM{
    fn new(filepath:String)->Option<Self>{
        if Path::new(&filepath).exists(){
            Some(PM{filepath, entries:vec![], cipher:None})
        }
        else{
            println!("path did not exist");
            None
        }
    }
   
   fn default()->Self{
        let cur_dir = env::current_dir().expect("unable to get current directory");
        let pstring = &(cur_dir.to_str().unwrap().to_owned() + &"/PMfiles/safe.pswd".to_string());
        let path = Path::new(pstring);
        //if the path doesn't exist here create a new directory and a save file within it
        let mut pass: Option<String> = None;
        if !path.exists(){
            println!("creating new file for entries...");
            fs::create_dir(cur_dir.to_str().unwrap().to_owned()+&"/PMfiles");
            let mut file = fs::File::create(path).expect("unable to create save file");
            pass = Some(helper::take_password_input());
            file.write_all("---START OF HASH SIGNATURE---END OF HASH SIGNATURE---\n".as_bytes());
        }
        let pm =  PM{filepath:path.to_str().unwrap().to_string(), entries:vec![], cipher:None};
        //we want to generate a hash the first time a person creates this file, so that the hash isn't just empty,
        //this way we can immediately get to using the hash if required
        match pass{
            Some(pass)=>{
                pm.generate_and_write_hash(pass);
            }
            None=>{

            }
        }
        pm
    }

    fn init_cipher(&mut self, key: Vec<u8>){
        match self.cipher{
            Some(_)=>{
                //no need to do anything
            }None=>{
                //initialise the cipher here
                self.cipher = Some(AesGcm::new(Key::<Aes256Gcm>::from_slice(&key.as_slice())));
                println!("cipher initialised, ready to encrypt, decrypt");
            }
        }
    }   

    fn take_new_entry_input(&mut self){
        let mut input = None;
        println!("You can type in 'quit' anytime to quit the action");
        println!("enter the entry name: ");
        input = take_input("Enter the entry name: ".to_string(), false);
        if input.is_none(){
            // entry name is a non-null value
            println!("Aborting command");
            return;
        }
        let entry_name = input.clone().unwrap();
        println!("enter any notes for this entry, or just press enter to continue: ");
        input = take_input("Enter any notes for this entry, or just press enter to continue: ".to_string(), false);
        let mut entry_note =input.clone();
        println!("enter the entry username: ");
        input = take_input("Enter the entry's username".to_string(), false);
        let entry_user_name = input.clone();
        println!("enter the entry password: ");
        // whilst it is actually a password input, it will be treated as a plain text at this stage, it will be changed later if required
        input = take_input("Enter the entry's password (no password is accepted)".to_string(), false);
        let entry_pass = input.clone();
        self.entries.push(Entry::new(entry_name,entry_user_name, entry_pass, entry_note));
    }

    /*
        below are the functions associated with pbkdf usage
        -> they all seem to work with a test password and a test hash
     */
    fn verify_password(&mut self,pass:String){
        let first_line = self.read_first_line();
        let split = PM::split_first_line(general_purpose::STANDARD.decode(first_line[29..first_line.len()-27].to_string()).unwrap());
        let salt = split.0;
        let enc_key = split.1;
        let mut kek2 = [0u8;32];
        pbkdf2_hmac::<Sha256>(pass.as_bytes(), &salt.as_slice(), 600_000, &mut kek2);

        let cipher2 = Aes256Gcm::new_from_slice(&kek2).expect("unable to generate key from slice given");
        match cipher2.decrypt(split.2.as_slice().into(), enc_key.as_slice()){
            Ok(key)=>{
                //lets initialise cipher here
                self.init_cipher(key);
            }Err(_)=>{
                println!("incorrect password!");
                println!("got hash: {kek2:?}");
            }
        }   
    }

    ///used to generate a random hash given a password, it will generate the hash, and then write the salt, encrypted actual key and nonce
    ///to file
    fn generate_and_write_hash(&self, password:String){ 
        let mut kek = [0u8;32];
        let mut salt = [0u8;32];
        OsRng.fill_bytes(&mut salt);
        pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, 600_000, &mut kek);
        let cipher = Aes256Gcm::new_from_slice(&kek).expect("unable to generate key from slice given");
        let nonce = Aes256Gcm::generate_nonce(OsRng);
        let mut enc_key = cipher.encrypt(&nonce, Aes256Gcm::generate_key(OsRng).as_slice()).expect("unable to encrypt key");
        let mut to_write = salt.to_vec();
        to_write.append(&mut enc_key);
        to_write.append(&mut nonce.to_vec());
        println!("hash generated: {kek:?}");
        self.write_first_line_to_file(general_purpose::STANDARD.encode(to_write));
    }

    fn read_first_line(&self)->String{
        read_to_string(&self.filepath).unwrap().lines().next().unwrap().to_string()
    }   
    
    fn write_first_line_to_file(&self, to_write:String){
        ///use this function to write the salt and nonce of the KEK to the file
        let mut file = File::options().read(true).open(Path::new(&self.filepath)).expect("unable to open file");
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(&mut buf);
        let mut contents = String::from_utf8(buf).unwrap();
        let x= contents.find("---END OF HASH SIGNATURE").expect("hash signature not found");
        let contents = contents.split_at(x).1.to_string();
        let to_write = "---START OF HASH SIGNATURE---" .to_string()+& to_write + "---END OF HASH SIGNATURE";
        let contents = to_write + "\n" + &contents[24..];
        let mut file = File::options().truncate(true).write(true).open(Path::new(&self.filepath)).expect("unable to open file");
        file.write_all(contents.as_bytes());
    }

    fn split_first_line(line:Vec<u8>)->(Vec<u8>,Vec<u8>,Vec<u8>){
        let split = line.split_at(line.len()-12);
        let split_other = split.0.split_at(32);
        (split_other.0.to_vec(),split_other.1.to_vec(),split.1.to_vec())
    }
    
    fn split_cipher_text(cipher_text:Vec<u8>)->(Vec<u8>,Vec<u8>){
        let split: (&[u8], &[u8]) =   cipher_text.split_at(cipher_text.len()-12);
        (split.0.to_vec(), split.1.to_vec())
    }

    /*basic encrypt decrypt functionality below*/

    fn encrypt(& self, plain_text:String)->Option<Vec<u8>>{
        match  &self.cipher{
            Some(cipher)=>{
                let nonce = Aes256Gcm::generate_nonce(OsRng);
                let mut cipher_text = cipher.encrypt(&nonce, plain_text.as_bytes()).expect("unable to encrypt text");
                cipher_text.append(&mut nonce.to_vec());
                Some(cipher_text)
            }None=>{
                println!("cipher was not initialised");
                None
            }
        }

    }
    
    fn decrypt(&self, cipher_text:Vec<u8>)->Option<Vec<u8>>{
        match &self.cipher {
            Some(cipher)=>{
                let split = PM::split_cipher_text(cipher_text);
                Some(cipher.decrypt(GenericArray::from_slice(split.1.as_slice()), split.0.as_slice()).expect("unable to decrypt"))
            }
            None=>{
                println!("cipher was not initialised");
                None
            }
        }
    }
    
    /*do not use obsfucating functions yet */
    
    fn obsfucate_hash(hash:String){
        let hash = hash.as_bytes();
        let len: usize = hash.len();
        let obsfucated_len = len*10;
        let mut obsfucated_hash:Vec<u8> = vec![];
        for x in 0..obsfucated_len{
            if x%10 == 0{
                obsfucated_hash.push(hash[x]);
            }else{
                obsfucated_hash.push(rand::thread_rng().gen_range(0..=255));
            }
        }
    }

    fn deobsfucate_hash(hash:Vec<u8>)->Vec<u8>{
        let len = hash.len();
        if len%10 !=0{
            println!("invalid obsfucated hash");
            return vec![];
        }else{
            let len= len /10;
            let mut ret:Vec<u8>= vec![];
            for x in 0..len{
                ret.push(hash[x*10]);
            }
            return vec![]
        }
    }

    /* below are the two basic read/write functions */
    fn write_entries_to_file(&self){
        let mut file = File::options().read(true).write(true).open(&self.filepath).expect("unable to open file for writing");
        let mut to_write = self.read_first_line();
        to_write += "\n";
        for entry in &self.entries{
            let mut entry = self.encrypt(String::from_utf8(entry.stringify().into()).unwrap()).expect("unable to encrypt");
            to_write += &String::from_utf8(general_purpose::STANDARD.encode(entry).as_bytes().to_vec()).expect("unable to stringify base64");
            to_write += "\n";
        }
        to_write.pop();
        file.write_all(to_write.as_bytes());
        println!("successfully wrote {:?} entries to file", self.entries.len());
    }
    
    fn read_entries_from_file(&mut self){
        let mut file: File = File::open(&self.filepath).expect("unable to open entries file to read");
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(&mut buf).expect("unable to read from file");
        let mut contents: String = String::from_utf8(buf).expect("unable to stringify");
        let mut contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();
        //remove the hash from the contents
        contents.remove(0);
        for line in contents{
            self.entries.push(
                Entry::parse(
       String::from_utf8(
                    self.decrypt(
                general_purpose::STANDARD.decode(line).expect("unable to decode base64 bytes for decrypting"))
                        .expect("unable to decrypt"))
                    .expect("unable to stringify plaintext"))
                .unwrap()
            );
        }
    }

    /* */
    fn display_entries(&self){
        println!("displaying entries...");
        for entry in &self.entries{
            entry.display();
        }
    }
    fn print_menu_options(width: usize){
        let other_width = width +2;
        println!("{: ^width$}","Menu Options");
        println!("{:-^other_width$}","");
        println!("1.{:_>width$}"," Display entries");
        println!("2.{:_>width$}"," Add entry");
        println!("3.{:_>width$}"," Edit entry");
        println!("4.{:_>width$}"," Delete entry");
        println!("5.{:_>width$}"," Save and exit");
        println!("6.{:_>width$}"," Exit NO SAVE");
    }

    fn menu(&mut self){
        let width = termion::terminal_size().unwrap().0 as usize/3;
        let mut input = "".to_string();
        println!("{:#^width$}"," Password Manager ");
        println!();
        loop{
            // password verification loop, only exit upon the user directly wanting to quit or no password entry
            let pass = helper::take_password_input();
            if pass == ""{
                println!("Exiting... no password given");
                exit(0);
            }
            self.verify_password(pass);
            // we break if the cipher is set (password was accepted)
            if self.cipher.is_some(){
                break;
            }
        }
        self.read_entries_from_file();
        // main loop
        loop {
            /*
                #######    ######    #######   #######  ##              ##   #########  #######   ########## \n
                ##    ##  ##    ##  ##        ##        ##              ##  ##       ## ##    ##  ##       ##\n
                ##    ##  ##    ##  ##        ##        ##              ##  ##       ## ##    ##  ##       ##\n
                #######   ########   ######    ######   ##      #       ##  ##       ## #######   ##       ##\n
                ##        ##    ##        ##        ##  ##     # #      ##  ##       ## ####      ##       ##\n
                ##        ##    ##        ##        ##  ##    #   #     ##  ##       ## ##  ##    ##       ##\n
                ##        ##    ##  #######   #######    ######    ######   #########   ##    ##  ##########\n


                ######    ####
                ##   #########
                ##   #########
                ##   #########
                ##   #########
                ##   #########
                ##   #########
             */
            
            PM::print_menu_options(width);
            stdin().read_line(&mut input).expect("unable to take input for menu");
            input = input[..input.len()-1].to_string();
            if input == "1".to_string(){
                self.display_entries();
            }else if input == "2".to_string(){
                println!("Search ");

            }
            else if input == "3".to_string(){
                println!("Add new entry");
                self.take_new_entry_input();
            }else if input == "4".to_string(){
                println!("Edit entry");
            }else if input == "5".to_string(){
                println!("Delete entry");
            }else if input == "6".to_string(){
                println!("Saving entries...");
                self.write_entries_to_file();
                println!("Exiting...");
                exit(0);
            }else if input == "6".to_string(){
                exit(0);
            }else{
                println!("Invalid input");
            }
            break;
        }
    }

}



fn main(){
    let mut pm = PM::new("PMfiles/safe.pswd".to_string()).unwrap();
    pm.menu();
    // pm.verify_password("asdf1234".to_string());
    // pm.write_entries_to_file();
    // pm.read_entries_from_file();
    // pm.display_entries();
}