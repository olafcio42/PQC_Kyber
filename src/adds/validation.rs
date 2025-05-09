use anyhow::{Result, bail};
use pqcrypto_kyber::kyber1024::{PublicKey, SecretKey, encapsulate, decapsulate};
use pqcrypto_traits::kem::SharedSecret;

pub fn validate_keys(pk: &PublicKey, sk: &SecretKey) -> Result<()> {
    let (ss1, ct) = encapsulate(pk);
    let ss2 = decapsulate(&ct, sk);

    if ss1.as_bytes() != ss2.as_bytes() {
        bail!("Klucze nie są kompatybilne!")
    }

    Ok(())
}