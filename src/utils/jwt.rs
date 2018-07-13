use std::result;
use std::error;
use std::fmt;
use std::string;

use serde::Serialize;
use serde::de::DeserializeOwned;
use ring::{ self, digest };
use ring::hmac;
use data_encoding::{ self, BASE64URL_NOPAD };
use serde_json;


type Result<T> = result::Result<T, Error>;

pub enum Algorithm {
    SHA1,
    SHA256,
    SHA384,
    SHA512
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    alg: String,
    typ: String
}

impl Header {
    fn new(alg: Algorithm) -> Header {
        
        let algorithm = match alg {
            Algorithm::SHA1 => "HS1".to_string(),
            Algorithm::SHA256 => "HS256".to_string(),
            Algorithm::SHA384 => "HS384".to_string(),
            Algorithm::SHA512 => "HS512".to_string()
        };

        Header{
            alg: algorithm,
            typ: "JWT".to_string(),
        }
    }

    fn algorithm(&self) -> &'static digest::Algorithm {

        match &*self.alg {
            "HS1" => &digest::SHA1,
            "HS256" => &digest::SHA256,
            "HS384" => &digest::SHA384,
            "HS512" => &digest::SHA512,
            _ => &digest::SHA256
        }
    }

    fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    fn from_str(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    fn encode_base64(&self) -> Result<String> {
        Ok(BASE64URL_NOPAD.encode(self.to_string()?.as_bytes()))
    }

    fn decode_base64(base: &str) -> Result<Self> {
        let _json = BASE64URL_NOPAD.decode(base.as_bytes())?;
        Header::from_str(&String::from_utf8(_json)?)
    }
}

pub trait Message: Serialize + DeserializeOwned {
    fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    fn from_str(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    fn encode_base64(&self) -> Result<String> {
        Ok(BASE64URL_NOPAD.encode(self.to_string()?.as_bytes()))
    }

    fn decode_base64(base: &str) -> Result<Self> {
        let _json = BASE64URL_NOPAD.decode(base.as_bytes())?;
        Message::from_str(&String::from_utf8(_json)?)
    }
}

pub fn encode<M>(key: &str, message: M, alg: Algorithm) -> Result<String> where M: Message {
    let message_base64 = message.encode_base64()?;

    let header = Header::new(alg);

    let unsigned_token = header.encode_base64()? + "." + &message_base64;

    let signature = hmac::sign(
        &hmac::SigningKey::new(header.algorithm(), key.as_bytes()),
        unsigned_token.as_bytes()
    );

    let signature_base64 = BASE64URL_NOPAD.encode(signature.as_ref());

    Ok(unsigned_token + "." + &signature_base64)
}

pub fn decode<M>(key: &str, token: String) -> Result<M> where M: Message {
    let token_split: Vec<&str> = token.split('.').collect();

    if token_split.len() != 3 {
        return Err(JwtError::Decode.into())
    }

    let header_base64 = token_split[0];
    let message_base64 = token_split[1];

    hmac::verify(
        &hmac::VerificationKey::new(
            Header::decode_base64(&header_base64)?.algorithm(),
            key.as_bytes()
        ), 
        (header_base64.to_string() + "." + &message_base64).as_bytes(),
        &BASE64URL_NOPAD.decode(token_split[2].as_bytes())?
    ).or(Err(JwtError::Verify))?;

    Message::decode_base64(message_base64)
}



#[derive(Debug)]
pub enum JwtError {
    Decode,
    Verify
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Token decode error")
    }
}

impl error::Error for JwtError {
    fn description(&self) -> &str {
        "Token decode error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Crypto(ring::error::Unspecified),
    Base64(data_encoding::DecodeError),
    FromUtf8Error(string::FromUtf8Error),
    JwtError(JwtError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Json(ref err) => write!(f, "Json error: {}", err),
            Error::Crypto(ref err) => write!(f, "Crypto error: {}", err),
            Error::Base64(ref err) => write!(f, "Base64 error: {}", err),
            Error::FromUtf8Error(ref err) => write!(f, "FromUtf8 error: {}", err),
            Error::JwtError(ref err) => write!(f, "Jwt error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Json(ref err) => err.description(),
            Error::Crypto(ref err) => err.description(),
            Error::Base64(ref err) => err.description(),
            Error::FromUtf8Error(ref err) => err.description(),
            Error::JwtError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Json(ref err) => Some(err),
            Error::Crypto(ref err) => Some(err),
            Error::Base64(ref err) => Some(err),
            Error::FromUtf8Error(ref err) => Some(err),
            Error::JwtError(ref err) => Some(err)
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<ring::error::Unspecified> for Error {
    fn from(err: ring::error::Unspecified) -> Self {
        Error::Crypto(err)
    }
}

impl From<data_encoding::DecodeError> for Error {
    fn from(err: data_encoding::DecodeError) -> Self {
        Error::Base64(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::FromUtf8Error(err)
    } 
}

impl From<JwtError> for Error {
    fn from(err: JwtError) -> Self {
        Error::JwtError(err)
    }
}
