use crate::crypto::ssh;
use crate::storage::vault;
use crate::crypto::encrypt;
use rpassword::prompt_password;

pub fn generate(name:String){
    println!("Generating Key for {}",name);
    vault::init_vault();

    let(private_key, public_key) = ssh::generate_ed25519_key(&name);

    let password = prompt_password("Enter master password: ").expect("Failed to read password");

    let encrypted_private_key = encrypt::encrypt_private_key(&private_key, &password,);

    vault::save_key(&name, &encrypted_private_key, &public_key);
    
   println!("\n✅ Encrypted key saved successfully!");

    println!("\n🔓 Public Key:\n{}\n", public_key);

}

pub fn list(){
    let keys = vault::list_keys();

    if keys.is_empty(){
        println!("No keys found");
        return;
    }
    println!("Stored keys:\n");
    
    for key in keys{
        println!("- {}",key);
    }
}

pub fn get(name:String,public:bool){
let key_data = vault::get_key(&name, public);

if public{
    println!("{}",key_data);
    return;
}

let password = prompt_password("
Enter the Master Password: ").expect("Failed to read password");

let decrypted_key = encrypt::decrypt_private_key(&key_data, &password,);

println!("{}",decrypted_key)

}
pub fn delete(name: String) {
    vault::delete_key(&name);

    println!("✅ Deleted key '{}'", name);
}