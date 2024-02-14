use std::ops::Add;

use crate::error::ContractError;
use crate::error::ContractError::{InvalidJWTAud, InvalidToken};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use cosmwasm_schema::cw_serde;
use phf::{phf_map, Map};
use rsa::traits::SignatureScheme;
use rsa::{BigUint, Pkcs1v15Sign, RsaPublicKey};

use sha2::{Digest, Sha256};

static AUD_KEY_MAP: Map<&'static str, &'static str> = phf_map! {
    "project-live-7e4a3221-79cd-4f34-ac1d-fedac4bde13e" => "7DEDs11mtM85pjdpELjoNBqBPcPf3rUU7llkoycaUfhlQF3ghMVBrIoVs4ivaBGJiBGBEnM64lKeCMYDaTDa67AUsUIahyBtKTHvZ_tEgOiqX6feWg-z6MsoA7HFoxbIzgwTGEVcFzy5y0BQEqffPstSBLUeZRfh7NGSXbGoo5zXPx1oEgrFtzfpnBgz-OP2rg1JLdycMP3YoKFIu5v2nnRobvlEraXil3ETJ-c6TLcaOctd1T4HSFNk5xy7HqiqMqU4Ixy5HfzC7gJqo1g1ppPrkSY36hpPgtpa6xR161cPr9Acvejqt8LK5xpoeW8oS67r1_m-TkKjTOhKzjbVNw;AQAB",
};
pub struct Token {
    //header: String,
    payload: String,
    digest: String,
    signature: String,
}
#[cw_serde]
pub struct Payload {
    pub email_address: String,
    pub xion_address: String,
    pub aud: String,
}

impl Token {
    pub fn verify(aud: &str, jwt: &str) -> Result<Payload, ContractError> {
        let key = Self::verify_audience(&aud)?;
        let _token = match Self::from_string(&jwt) {
            Some(t) => Ok(t),
            None => Err(ContractError::InvalidToken),
        }?;
        Self::validate_signature(&_token, &key)?;
        Self::extract_payload(&_token)
    }
    pub fn from_string(token: &str) -> Option<Self> {
        let parts: Vec<&str> = token.split('.').collect();
        match parts.len() {
            3 => Some(Self {
                //header: parts[0].to_string(),
                payload: parts[1].to_string(),
                digest: [parts[0], ".", parts[1]].concat(),
                signature: parts[2].to_string(),
            }),
            _ => None,
        }
    }

    pub fn validate_signature(&self, key: &str) -> Result<(), ContractError> {
        let signature = URL_SAFE_NO_PAD
            .decode(&self.signature)
            .map_err(|_| ContractError::InvalidToken)?;

        let pubkey = Self::build_pubkey(&key).map_err(|_| ContractError::InvalidKey)?;

        // hash the message body before verification
        let mut hasher = Sha256::new();
        hasher.update(&self.digest);
        let digest = hasher.finalize().as_slice().to_vec();

        // verify the signature
        let scheme = Pkcs1v15Sign::new::<Sha256>();
        scheme
            .verify(&pubkey, digest.as_slice(), signature.as_slice())
            .map_err(|_| ContractError::InvalidToken)?;

        Ok(())
    }

    fn verify_audience(aud: &str) -> Result<String, ContractError> {
        if !AUD_KEY_MAP.contains_key(aud) {
            return Err(InvalidJWTAud);
        }

        let key = match AUD_KEY_MAP.get(aud) {
            None => return Err(InvalidJWTAud),
            Some(k) => *k,
        };
        Ok(key.to_string())
    }

    fn build_pubkey(key: &str) -> Result<RsaPublicKey, ContractError> {
        let mut key_split = key.split(';');
        let modulus = key_split.next().ok_or(ContractError::InvalidKey)?;
        let mod_bytes = URL_SAFE_NO_PAD
            .decode(modulus)
            .map_err(|_| ContractError::InvalidKey)?;
        let exponent = key_split.next().ok_or(ContractError::InvalidKey)?;
        let exp_bytes = URL_SAFE_NO_PAD
            .decode(exponent)
            .map_err(|_| ContractError::InvalidKey)?;
        let pubkey = RsaPublicKey::new(
            BigUint::from_bytes_be(mod_bytes.as_slice()),
            BigUint::from_bytes_be(exp_bytes.as_slice()),
        )
        .map_err(|_| ContractError::InvalidKey)?;

        Ok(pubkey)
    }

    fn extract_payload(&self) -> Result<Payload, ContractError> {
        // decode base64
        let decoded_payload = URL_SAFE_NO_PAD
            .decode(&self.payload)
            .map_err(|_| ContractError::InvalidToken)?;
        let utf8_str = String::from_utf8_lossy(&decoded_payload).to_string();

        let aud = Self::extract_aud(&utf8_str)?;

        let start_index = Self::get_colon_index(&aud)?;
        let end_index = Self::get_colon_index(&aud[1.add(start_index)..])?;

        let _payload = Payload {
            email_address: Self::extract_string(&utf8_str, "\"email_address\":\"")?,
            xion_address: Self::extract_string(&utf8_str, "\"xion_address\":\"")?,
            aud: aud[start_index..end_index].to_string(),
        };

        Ok(_payload)
    }

    fn extract_string(_str: &str, _find: &str) -> Result<String, ContractError> {
        let start_index = match &_str.find(_find) {
            Some(index) => index + _find.len(),
            None => {
                return Err(ContractError::InvalidToken);
            }
        };
        let end_index = Self::get_colon_index(&_str[start_index..])?;

        Ok(_str[start_index..end_index.add(start_index)].to_string())
    }

    fn extract_aud(_str: &str) -> Result<String, ContractError> {
        let _find = "\"aud\":";
        let start_index = match &_str.find(_find) {
            Some(index) => index + _find.len(),
            None => {
                return Err(ContractError::InvalidToken);
            }
        };
        let end_index = _str.len() - 1;
        Ok(_str[start_index..end_index].to_string())
    }

    fn get_colon_index(_str: &str) -> Result<usize, ContractError> {
        match _str.find('"') {
            Some(index) => Ok(index),
            None => Err(ContractError::InvalidToken),
        }
    }
}

pub fn verify(
    //current_time: &Timestamp,
    //tx_hash: &Vec<u8>,
    sig_bytes: &[u8],
    aud: &str,
    // sub: &str,
) -> Result<String, ContractError> {
    if !AUD_KEY_MAP.contains_key(aud) {
        return Err(InvalidJWTAud);
    }

    let key = match AUD_KEY_MAP.get(aud) {
        None => return Err(InvalidJWTAud),
        Some(k) => *k,
    };

    // prepare the components of the token for verification
    let mut components = sig_bytes.split(|&b| b == b'.');
    let header_bytes = components.next().ok_or(InvalidToken)?; // ignore the header, it is not currently used
    let payload_bytes = components.next().ok_or(InvalidToken)?;
    let digest_bytes = [header_bytes, &[b'.'], payload_bytes].concat();
    let signature_bytes = components.next().ok_or(InvalidToken)?;

    let signature = URL_SAFE_NO_PAD
        .decode(signature_bytes)
        .map_err(|_| ContractError::InvalidToken)?;

    // retrieve and rebuild the pubkey
    let mut key_split = key.split(';');
    let modulus = key_split.next().ok_or(InvalidJWTAud)?;
    let mod_bytes = URL_SAFE_NO_PAD
        .decode(modulus)
        .map_err(|_| ContractError::InvalidToken)?;
    let exponent = key_split.next().ok_or(InvalidJWTAud)?;
    let exp_bytes = URL_SAFE_NO_PAD
        .decode(exponent)
        .map_err(|_| ContractError::InvalidToken)?;
    let pubkey = RsaPublicKey::new(
        BigUint::from_bytes_be(mod_bytes.as_slice()),
        BigUint::from_bytes_be(exp_bytes.as_slice()),
    )
    .map_err(|_| ContractError::InvalidToken)?;

    // hash the message body before verification
    let mut hasher = Sha256::new();
    hasher.update(digest_bytes);
    let digest = hasher.finalize().as_slice().to_vec();

    // verify the signature
    let scheme = Pkcs1v15Sign::new::<Sha256>();
    scheme
        .verify(&pubkey, digest.as_slice(), signature.as_slice())
        .map_err(|_| ContractError::InvalidToken)?;

    // at this point, we have verified that the token is legitimately signed.
    // now we perform logic checks on the body

    extract_email(&payload_bytes)
}

fn extract_email(payload_bytes: &[u8]) -> Result<String, ContractError> {
    let decoded_payload = URL_SAFE_NO_PAD
        .decode(payload_bytes)
        .map_err(|_| ContractError::InvalidToken)?;

    let st = String::from_utf8_lossy(&decoded_payload);

    let start_index = match st.find("\"email_address\":\"") {
        Some(index) => index + "\"email_address\":\"".len(),
        None => {
            return Err(ContractError::InvalidToken);
        }
    };

    let end_index = match st[start_index..].find('"') {
        Some(index) => index + start_index,
        None => {
            return Err(ContractError::InvalidToken);
        }
    };

    let email_address = &st[start_index..end_index];

    Ok(email_address.to_owned())
}
