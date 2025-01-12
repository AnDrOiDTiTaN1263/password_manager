pub struct Entry{
    pub entry_name: String,
    pub entry_user_name: Option<String>,
    pub entry_pass:Option<String>,
    pub entry_note:Option<String>,
}
#[allow(dead_code,unused)]
impl Entry{
    ///creates a new entry with all values filled with values (some can be None)
    pub fn new(entry_name:String, entry_user_name:Option<String>, entry_pass:Option<String>, entry_note:Option<String>)->Self{
        Self { entry_name, entry_user_name, entry_pass, entry_note}
    }
    
    ///used when only the entry name has been saved and nothing else
    pub fn empty(entry_name:String)->Self{
        Self { entry_name, entry_user_name: None, entry_pass: None, entry_note: None }
    }
    
    ///used for taking notes only, does not contain a password, just the entry name and the note
    pub fn note(entry_name:String, entry_note:String)->Self{
        Self { entry_name, entry_user_name: None, entry_pass: None, entry_note:Some(entry_note) }
    }
    
    ///pretty printer of Entry should show something like:
    /// 
    /// Entry name: ABC
    /// 
    ///     User: DEF
    /// 
    ///     Pass: GHI
    /// 
    ///     note: JKLM NOP
    pub fn display(&self){
        println!("Entry name: {}\n\tUser: {}\n\tPass:{}\n\tnote:{}", self.entry_name, self.entry_user_name.clone().unwrap_or("N/A".to_string()), self.entry_pass.clone().unwrap_or("N/A".to_string()), self.entry_note.clone().unwrap_or("N/A".to_string()));
    }
    ///converts Entry to a string for encryption
    pub fn stringify(&self)->String{
        return self.entry_name.clone() + "$" + &self.entry_user_name.clone().unwrap_or("N/A".to_string()) + "$" + &self.entry_pass.clone().unwrap_or("N/A".to_string()) + "$" + &self.entry_note.clone().unwrap_or("N/A".to_string());
    }
    //converts string to Entry (if possible) for storage
    pub fn parse(entry_string: String)->Option<Entry>{
        let split = entry_string.split("$").collect::<Vec<&str>>();
        if split.len()==4{
            return Some(Entry { entry_name: split[0].to_string(), entry_user_name: Some(split[1].to_string()), entry_pass: Some(split[2].to_string()), entry_note: Some(split[3].to_string()) });
        }
        println!("given string did not have enough features to populate an entry");
        None
    }
}
