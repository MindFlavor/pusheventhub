use std::convert::From;

use url::ParseError;
use std::io::Error as IOError;
use hyper::error::Error as HyperError;
use hyper::status::StatusCode;

#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedHTTPResult {
    expected: StatusCode,
    received: StatusCode,
    body: String,
}

impl UnexpectedHTTPResult {
    pub fn new(expected: StatusCode, received: StatusCode, body: &str) -> UnexpectedHTTPResult {
        UnexpectedHTTPResult {
            expected: expected,
            received: received,
            body: body.to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum AzureError {
    ParseError(ParseError),
    IOError(IOError),
    HyperError(HyperError),
    UnexpectedHTTPResult(UnexpectedHTTPResult),
}

impl From<ParseError> for AzureError {
    fn from(pe: ParseError) -> Self {
        AzureError::ParseError(pe)
    }
}

impl From<IOError> for AzureError {
    fn from(e: IOError) -> Self {
        AzureError::IOError(e)
    }
}

impl From<HyperError> for AzureError {
    fn from(e: HyperError) -> Self {
        AzureError::HyperError(e)
    }
}
