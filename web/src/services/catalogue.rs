use alloc::{format, string::String, vec::Vec};
use core::fmt;
use gloo::net::http::Request;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CatalogueLoadError {
    Request(String),
    Http(u16),
    Body(String),
    Invalid(String),
}

impl fmt::Display for CatalogueLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(details) => write!(
                formatter,
                "Could not request the language catalogue: {details}"
            ),
            Self::Http(status) => write!(
                formatter,
                "Could not load the language catalogue (HTTP {status})."
            ),
            Self::Body(details) => write!(
                formatter,
                "Could not read the language catalogue response: {details}"
            ),
            Self::Invalid(details) => write!(formatter, "Invalid language catalogue: {details}"),
        }
    }
}

impl core::error::Error for CatalogueLoadError {}
