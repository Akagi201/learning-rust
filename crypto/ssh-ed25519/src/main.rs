use signature::{Signer, Verifier};
use ssh_key::{private::KeypairData, public::KeyData, PrivateKey, PublicKey};
const ED25519_PRIVATE_KEY: &str = include_str!("id_ed25519");
const ED25519_PUBLIC_KEY: &str = include_str!("id_ed25519.pub");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openssh_private_key: PrivateKey = PrivateKey::from_openssh(ED25519_PRIVATE_KEY).unwrap();
    let ed25519_keypair = openssh_private_key.key_data();
    if let KeypairData::Ed25519(ed25519_keypair) = ed25519_keypair {
        let public_key = ed25519_keypair.clone().public;
        println!("Public key: {}", public_key);

        let private_key = ed25519_keypair.private.clone();
        println!("Private key: {:?}", private_key);

        let msg = "hello";
        let signature = ed25519_keypair.sign(msg.as_bytes());
        // verify signature
        assert!(public_key.verify(msg.as_bytes(), &signature).is_ok());
    }

    let openssh_public_key: PublicKey = PublicKey::from_openssh(ED25519_PUBLIC_KEY).unwrap();
    let ed25519_public_key = openssh_public_key.key_data();
    if let KeyData::Ed25519(ed25519_public_key) = ed25519_public_key {
        println!("Public key: {}", ed25519_public_key);
    }

    Ok(())
}
