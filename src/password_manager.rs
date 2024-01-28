#[allow(dead_code, unused, unused_imports)]
use aead::{OsRng, KeyInit, Aead, AeadCore, generic_array::GenericArray, Key};
use aes_gcm::{Aes256Gcm, AesGcm};
use base64::{engine::general_purpose, Engine};
use pbkdf2::pbkdf2_hmac;
use crate::entry::Entry;

#[allow(dead_code, unused)]
pub struct PasswordManager{
    entries: Vec<Entry>,
    entries_filepath:String,
    cipher:Option<Aes256Gcm>,
}

impl PasswordManager{
    /*contains basic password manager functionality:
        encryption decryption
        
    */

}
