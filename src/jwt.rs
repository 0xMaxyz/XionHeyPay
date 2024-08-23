use crate::error::ContractError;
use crate::error::ContractError::InvalidJWTKid;
use crate::state::{AUDIENCE, KID_MAP};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Storage;
use rsa::traits::SignatureScheme;
use rsa::{BigUint, Pkcs1v15Sign, RsaPublicKey};

use sha2::{Digest, Sha256};

// static KID_MAP: Map<&'static str, &'static str> = phf_map! {
//     // "project-live-7e4a3221-79cd-4f34-ac1d-fedac4bde13e" => "7DEDs11mtM85pjdpELjoNBqBPcPf3rUU7llkoycaUfhlQF3ghMVBrIoVs4ivaBGJiBGBEnM64lKeCMYDaTDa67AUsUIahyBtKTHvZ_tEgOiqX6feWg-z6MsoA7HFoxbIzgwTGEVcFzy5y0BQEqffPstSBLUeZRfh7NGSXbGoo5zXPx1oEgrFtzfpnBgz-OP2rg1JLdycMP3YoKFIu5v2nnRobvlEraXil3ETJ-c6TLcaOctd1T4HSFNk5xy7HqiqMqU4Ixy5HfzC7gJqo1g1ppPrkSY36hpPgtpa6xR161cPr9Acvejqt8LK5xpoeW8oS67r1_m-TkKjTOhKzjbVNw;AQAB",
//     "09bcf8028e06537d4d3ae4d84f5c5babcf2c0f0a" => "vdtZ3cfuh44JlWkJRu-3yddVp58zxSHwsWiW_jpaXgpebo0an7qY2IEs3D7kC186Bwi0T7Km9mUcDbxod89IbtZuQQuhxlgaXB-qX9GokNLdqg69rUaealXGrCdKOQ-rOBlNNGn3M4KywEC98KyQAKXe7prs7yGqI_434rrULaE7ZFmLAzsYNoZ_8l53SGDiRaUrZkhxXOEhlv1nolgYGIH2lkhEZ5BlU53BfzwjO-bLeMwxJIZxSIOy8EBIMLP7eVu6AIkAr9MaDPJqeF7n7Cn8yv_qmy51bV-INRS-HKRVriSoUxhQQTbvDYYvJzHGYu_ciJ4oRYKkDEwxXztUew;AQAB",
//     "adf5e710edfebecbefa9a61495654d03c0b8edf8" => "y48N6JB-AKq1-Rv4SkwBADU-hp4zXHU-NcCUwxD-aS9vr4EoT9qrjoJ-YmkaEpq9Bmu1yXZZK_h_9QS3xEsO8Rc_WSvIQCJtIaDQz8hxk4lUjUQjMB4Zf9vdTmf8KdktI9tCYCbuSbLC6TegjDM9kbl9CNs3m9wSVeO_5JXJQC0Jr-Oj7Gz9stXm0Co3f7RCxrD08kLelXaAglrd5TeGjZMyViC4cw1gPaj0Cj6knDn8UlzR_WuBpzs_ies5BrbzX-yht0WfnhXpdpiGNMbpKQD04MmPdMCYq8ENF7q5_Ok7dPsVj1vHA6vFGnf7qE3smD157szsnzn0NeXIbRMnuQ;AQAB",
// };
// static AUDIENCE: &'static str =
//     "222037837154-pnh5rdr8d9hvfj9ioore6amb0gqs4bj9.apps.googleusercontent.com";
pub struct Token {
    header: Header,
    payload: Payload,
    digest: String,
    signature: String,
}
#[cw_serde]
pub struct Payload {
    iss: String,
    azp: String,
    aud: String,
    sub: String,
    pub email: String,
    email_verified: bool,
    pub nonce: String,
    nbf: u64,
    name: String,
    picture: String,
    given_name: String,
    iat: u64,
    exp: u64,
    jti: String,
}

#[cw_serde]
pub struct Header {
    pub alg: String,
    pub kid: String,
    pub typ: String,
}

impl Token {
    pub fn verify(
        jwt: &str,
        timestamp: u64,
        bypasstimestamp: bool,
        storage: &dyn Storage,
    ) -> Result<Payload, ContractError> {
        // deserialize token
        let _token = match Self::from_string(&jwt) {
            Some(t) => Ok(t),
            None => Err(ContractError::InvalidToken),
        }?;
        // get key using the kid
        let key = Self::get_key(&_token.header.kid, storage)?;
        let audience = match AUDIENCE
            .may_load(storage)
            .map_err(|_| ContractError::InvalidAudience)?
        {
            Some(aud) => aud,
            None => Err(ContractError::InvalidAudience)?,
        };
        // validate audience
        if _token.payload.aud != audience {
            return Err(ContractError::InvalidEmail);
        }
        // validate timestamp
        if !bypasstimestamp && (timestamp < _token.payload.iat || timestamp > _token.payload.exp) {
            return Err(ContractError::ExpiredToken);
        }
        // validate the signature
        Self::validate_signature(&_token, &key)?;
        Ok(_token.payload)
    }
    pub fn from_string(token: &str) -> Option<Self> {
        let parts: Vec<&str> = token.split('.').collect();
        match parts.len() {
            3 => Some(Self {
                header: Self::deserialize_header(parts[0]).unwrap(),
                payload: Self::deserialize_payload(parts[1]).unwrap(),
                digest: [parts[0], ".", parts[1]].concat(),
                signature: parts[2].to_string(),
            }),
            _ => None,
        }
    }

    pub fn validate_signature(&self, key: &str) -> Result<(), ContractError> {
        let signature = Self::convert_from_base64(&self.signature, ContractError::InvalidToken)?;

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

    fn get_key(kid: &str, storage: &dyn Storage) -> Result<String, ContractError> {
        // Load keys
        let keys = KID_MAP
            .may_load(storage, kid)
            .map_err(|_| ContractError::InvalidJWTKid)?;
        let key = match keys {
            None => return Err(InvalidJWTKid),
            Some(k) => k,
        };

        Ok(key.to_string())
    }

    fn build_pubkey(key: &str) -> Result<RsaPublicKey, ContractError> {
        // split the key
        let mut key_split = key.split(';');
        // extract the modulus and exponent
        let modulus_base64 = key_split.next().ok_or(ContractError::InvalidKey)?;
        let exponent_base64 = key_split.next().ok_or(ContractError::InvalidKey)?;
        // decode modulus and exponent
        let modulus_bytes = Self::convert_from_base64(modulus_base64, ContractError::InvalidKey)?;
        let exponent_bytes = Self::convert_from_base64(exponent_base64, ContractError::InvalidKey)?;
        // convert to BigUint
        let modulus = BigUint::from_bytes_be(modulus_bytes.as_slice());
        let exponent = BigUint::from_bytes_be(exponent_bytes.as_slice());
        // build the public key
        let pubkey = RsaPublicKey::new(modulus, exponent).map_err(|_| ContractError::InvalidKey)?;

        Ok(pubkey)
    }

    fn deserialize_payload(pl: &str) -> Result<Payload, ContractError> {
        let decoded_payload = Self::convert_from_base64(pl, ContractError::InvalidToken)?;

        let utf8_str = String::from_utf8_lossy(&decoded_payload).to_string();

        let payload: Payload =
            serde_json_wasm::from_str(&utf8_str).map_err(|_| ContractError::InvalidToken)?;

        Ok(payload)
    }

    fn deserialize_header(h: &str) -> Result<Header, ContractError> {
        let decoded_header = Self::convert_from_base64(h, ContractError::InvalidToken)?;

        let utf8_str = String::from_utf8_lossy(&decoded_header).to_string();

        let header: Header =
            serde_json_wasm::from_str(&utf8_str).map_err(|_| ContractError::InvalidToken)?;

        Ok(header)
    }

    pub fn convert_from_base64(input: &str, err: ContractError) -> Result<Vec<u8>, ContractError> {
        let output = URL_SAFE_NO_PAD.decode(input).map_err(|_| err)?;
        Ok(output)
    }
}
