use std::io;

/// A kitchen sink error type to hold all of the differen types of errors we might encounter
#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Io(io::Error),
    SerdeJson(serde_json::Error),
    Syn(syn::Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJson(err)
    }
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Error {
        Error::Syn(err)
    }
}
