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
    // GA - Testnet - Test  project
    "project-test-5ae234a7-6b74-46af-a7b7-969f3df38cc0" => "4ia1pODcj-BPNblyJ1ao1etK0VltRWQEmeoQtHaCWrOES-2BCFbcOBsDDxrXPzkTUK5j15fpMFbg36vDqXiYDNPHTp7WxUrOKOSyONk4gZUd626GZwKJBryMAhU7mBMByO56sLUHdDPajykYIlpHut75gDqipDI5QY9fh_piLh7OMy-MORaWdmkv1zFqLfjAr2GUKFmd7xiUAYTsjDClTTMn1rGskjBF8qPK9jDrPz9SEwN1n7N0JPsJVRqP6m5Yf_l9JWSKarSLbV9O0qMC7Nl0MpBKTw8HTVlwaBWF-5aGbg3dMQl8Cbn4vNUv-pPjrlvrpw2m_r0Gr5N9CBEKFQ;AQAB",
    // GA - Testnet - Live project
    "project-live-7e4a3221-79cd-4f34-ac1d-fedac4bde13e" => "7DEDs11mtM85pjdpELjoNBqBPcPf3rUU7llkoycaUfhlQF3ghMVBrIoVs4ivaBGJiBGBEnM64lKeCMYDaTDa67AUsUIahyBtKTHvZ_tEgOiqX6feWg-z6MsoA7HFoxbIzgwTGEVcFzy5y0BQEqffPstSBLUeZRfh7NGSXbGoo5zXPx1oEgrFtzfpnBgz-OP2rg1JLdycMP3YoKFIu5v2nnRobvlEraXil3ETJ-c6TLcaOctd1T4HSFNk5xy7HqiqMqU4Ixy5HfzC7gJqo1g1ppPrkSY36hpPgtpa6xR161cPr9Acvejqt8LK5xpoeW8oS67r1_m-TkKjTOhKzjbVNw;AQAB",
    // Exodvs - Test project
    "project-test-185e9a9f-8bab-42f2-a924-953a59e8ff94" => "sQKkA829tzjU2VA-INHvdrewkbQzjpsMn0PNM7KJaBODbB4ItZM4x1NVSWBiy2DGHkaDDvADRbbq1BZsC1iXVtIYm0AoD7x4QC1w89kp2_s0wmvUOSPiQZlYrgJqRDXirXJZX3MNku2McXbwdyPajDaR4nBBQOoUOF21CHqLDqBHs2R6tHyL80R_8mgueiqQ-4wg6SSVcB_6ZOh59vRcjKr34upKPWGQzvMGCkeTO9whzbIWbA1j-8ykiS63EhjWBZU_sSolsf1ZGq8peVrADDLhOvHtZxCZLKwB46k2kb8GKAWlO4wRP6BDVjzpnea7BsvZ6JwULKg3HisH9gzaiQ;AQAB",
    "integration-test-project" => "olg7TF3aai-wR4HTDe5oR-WRhEsdW3u-O3IJHl0BiHkmR4MLskHG9HzivWoXsloUBnBMrFNxOH0x5cNMI07oi4PeRbHySiogRW9CXPjJaNlTi-pT_IgKFsyJNXsLyzrnajLkDbQU6pRsHmNeL0hAOUv48rtXv8VVWWN8okJehD2q9N7LHoFAOmIUEPg_VTHTt8K__O-9eMZKN4eMjh_4-sxRX6NXPSPT87XRlrK4GZ4pUdp86K0tOFLhwO4Uj0JkMNfI82eVZ1tAbDlqjd8jFnAb8fWm8wtdaTNbL_AAXmbDhswwJOyrw8fARZIhrXSdKBWa6e4k7sLwTIy-OO8saebnlARsjGst7ZCzmw5KCm2ctEVl3hYhHwyXu_A5rOblMrV3H0G7WqeKMCMVSJ11ssrlsmfVhNIwu1Qlt5GYmPTTJiCgGUGRxZkgDyOyjFNHglYpZamCGyJ9oyofsukEGoqMQ6WzjFi_hjVapzXi7Li-Q0OjEopIUUDDgeUrgjbGY0eiHI6sAz5hoaD0Qjc9e3Hk6-y7VcKCTCAanZOlJV0vJkHB98LBLh9qAoVUei_VaLFe2IcfVlrL_43aXlsHhr_SUQY5pHPlUMbQihE_57dpPRh31qDX_w6ye8dilniP8JmpKM2uIwnJ0x7hfJ45Qa0oLHmrGlzY9wi-RGP0YUk;AQAB",
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
