use core::panic;
use std::{fs, path::PathBuf};
 
pub fn get_vault_path()->PathBuf{
    let mut path = dirs::home_dir().expect("Could not find home dir");

    path.push(".sshx");
    path.push("vault");
    path
}

pub fn init_vault(){
    let path = get_vault_path();

    if !path.exists(){
        fs::create_dir_all(&path).expect(
            "Failed to create vault dir"
        )
    }
}

pub fn save_key(name: &str, private_key: &str, public_key:&str){
    init_vault();
    let base_path = get_vault_path();

    let private_path = base_path.join(name);
    let public_path = base_path.join(format!("{}.pub", name));

     
    fs::write(&private_path, private_key).expect("Failed to save private key");
    fs::write(&public_path, public_key).expect("Failed to save public key");
}

pub fn list_keys()-> Vec<String>{

    init_vault();

    let path = get_vault_path();
    let mut keys = Vec::new();
    let entries = fs::read_dir(path).unwrap();

    for entry in entries{

        let entry = entry.expect("Failed to read entry");

        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy().to_string();

        if !file_name.ends_with(".pub"){
            keys.push(file_name);
        }
    }
    keys.sort();
    keys

}

pub fn get_key(name: &str, public:bool)-> String{
    let base_path = get_vault_path();
    
    let path = if public{
        base_path.join(format!("{}.pub",name))
    }else {
        base_path.join(name)
    };

    if !path.exists() {
        panic!("Key '{}' not found", name);
    }

    std::fs::read_to_string(path).expect("Failed to read key")
}

pub fn delete_key (name: &str){
    let base_path = get_vault_path();

    let private_path = base_path.join(name);
    let public_path = base_path.join(format!("{}.pub",name));

    if private_path.exists(){
        fs::remove_file(private_path).expect("Faild to delete Private Key");
    }   

    if public_path.exists(){
        fs::remove_file(public_path).expect("Failed To delete Public Key");
    }
}