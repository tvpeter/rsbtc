use crate::sha256::Hash;
use crate::util::Saveable;
use ecdsa::signature::Signer;
use ecdsa::signature::Verifier;
use ecdsa::{Signature as ECDSASignature, SigningKey, VerifyingKey};
use k256::Secp256k1;
use serde::{Deserialize, Serialize};
use spki::EncodePublicKey;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult, Write};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Signature(pub ECDSASignature<Secp256k1>);

impl Signature {
    // sign a crate::types::TransactionOutput from its sha256 hash
    pub fn sign_output(output_hash: &Hash, private_key: &PrivateKey) -> Self {
        let signing_key = &private_key.0;
        let signature: ECDSASignature<Secp256k1> = signing_key.sign(&output_hash.as_bytes());
        Signature(signature)
    }

    pub fn verify(&self, output_hash: &Hash, public_key: &PublicKey) -> bool {
        public_key
            .0
            .verify(&output_hash.as_bytes(), &self.0)
            .is_ok()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PublicKey(VerifyingKey<Secp256k1>);

impl Saveable for PublicKey {
    fn load<I: Read>(mut reader: I) -> IoResult<Self> {
        // read PEM-encoded public key into string
        let mut buff = String::new();
        reader.read_to_string(&mut buff)?;

        //decode public key from PEM
        let public_key = buff
            .parse()
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "failed to parse public key"))?;
        Ok(PublicKey(public_key))
    }

    fn save<O: Write>(&self, mut writer: O) -> IoResult<()> {
        let s = self.0.to_public_key_pem(Default::default()).map_err(|_| {
            IoError::new(IoErrorKind::InvalidData, "Failed to serialize public key")
        })?;
        writer.write_all(s.as_bytes())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PrivateKey(#[serde(with = "signkey_serde")] pub SigningKey<Secp256k1>);

mod signkey_serde {
    use k256::Secp256k1;
    use serde::Deserialize;

    pub fn serialize<S>(
        key: &super::SigningKey<super::Secp256k1>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<super::SigningKey<Secp256k1>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = Vec::<u8>::deserialize(deserializer)?;
        Ok(super::SigningKey::from_slice(&bytes).unwrap())
    }
}

impl PrivateKey {
    pub fn new_key() -> Self {
        PrivateKey(SigningKey::random(&mut rand::thread_rng()))
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(*self.0.verifying_key())
    }
}

impl Saveable for PrivateKey {
    fn load<I: Read>(reader: I) -> IoResult<Self> {
        ciborium::de::from_reader(reader).map_err(|_| {
            IoError::new(
                IoErrorKind::InvalidData,
                "Failed to deserialize private key",
            )
        })
    }

    fn save<O: Write>(&self, writer: O) -> IoResult<()> {
        ciborium::ser::into_writer(self, writer).map_err(|_| {
            IoError::new(IoErrorKind::InvalidData, "failed to serialize private key")
        })?;
        Ok(())
    }
}
