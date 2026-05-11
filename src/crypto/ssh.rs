 
use ssh_key::{private::PrivateKey};
use rand_core::OsRng;

pub fn generate_ed25519_key(_name: &str)->(String, String){
    let mut rng = OsRng;
    let private_key = PrivateKey::random(&mut rng, ssh_key::Algorithm::Ed25519).expect("Faild to generate Key");

    let private_key_str = private_key.to_openssh(ssh_key::LineEnding::LF)
    .expect("Failed to encode Private key")
    .to_string();

    let public_key = private_key.public_key();

    let public_key_str = public_key.to_string();

    (private_key_str,public_key_str)
}