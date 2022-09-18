use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ClientError;

impl Display for ClientError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "w3name client error")
  }
}

impl Error for ClientError {}

#[derive(Debug)]
pub struct HttpError;

impl Display for HttpError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "http error")
  }
}

impl Error for HttpError {}

#[derive(Debug)]
pub struct APIError(pub String);

impl Display for APIError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "api error: {}", self.0)
  }
}

impl Error for APIError {}

#[derive(Debug)]
pub struct UnexpectedAPIResponse;

impl Display for UnexpectedAPIResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "unexpected response from API, unable to parse error message"
    )
  }
}

impl Error for UnexpectedAPIResponse {}

#[derive(Debug)]
pub struct NameError;

impl Display for NameError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "name error")
  }
}

impl Error for NameError {}

#[derive(Debug)]
pub struct InvalidCidString;

impl Display for InvalidCidString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "invalid cid string")
  }
}

impl Error for InvalidCidString {}

#[derive(Debug)]
pub struct InvalidMulticodecCode;

impl Display for InvalidMulticodecCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "invalid multicodec code")
  }
}

impl Error for InvalidMulticodecCode {}

#[derive(Debug)]
pub struct InvalidCryptoKey;

impl Display for InvalidCryptoKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "invalid crypto key")
  }
}

impl Error for InvalidCryptoKey {}

#[derive(Debug)]
pub struct SigningError;

impl Display for SigningError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "signing error")
  }
}

impl Error for SigningError {}

#[derive(Debug)]
pub struct CborError;

impl Display for CborError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "cbor error")
  }
}

impl Error for CborError {}

#[derive(Debug)]
pub struct ProtobufError;

impl Display for ProtobufError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "protobuf error")
  }
}

impl Error for ProtobufError {}

#[derive(Debug)]
pub struct InvalidIpnsV1Signature;

impl Display for InvalidIpnsV1Signature {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "invalid IPNS signature (v1)")
  }
}

impl Error for InvalidIpnsV1Signature {}

#[derive(Debug)]
pub struct InvalidIpnsV2Signature;

impl Display for InvalidIpnsV2Signature {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "invalid IPNS signature (v2)")
  }
}

impl Error for InvalidIpnsV2Signature {}

#[derive(Debug)]
pub struct InvalidIpnsV2SignatureData;

impl Display for InvalidIpnsV2SignatureData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "invalid IPNS v2 signature data (does not match protobuf values)"
    )
  }
}

impl Error for InvalidIpnsV2SignatureData {}

#[derive(Debug)]
pub struct InvalidUtf8;

impl Display for InvalidUtf8 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "invalid utf-8")
  }
}

impl Error for InvalidUtf8 {}

#[derive(Debug)]
pub struct InvalidDateString;

impl Display for InvalidDateString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "invalid RFC-3339 date string")
  }
}

#[derive(Debug)]
pub struct IpnsError;

impl Display for IpnsError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "IPNS record error")
  }
}

impl Error for IpnsError {}
