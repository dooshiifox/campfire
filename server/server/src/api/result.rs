use crate::prelude::*;
use actix_web::HttpResponseBuilder;
use std::fmt::Debug;

/// A successful API response.
pub struct ApiSuccess<T: Serialize>(pub T);

impl<T: Serialize + Debug> ApiSuccess<T> {
    pub fn into_response(self, code: StatusCode) -> HttpResponse {
        HttpResponseBuilder::new(code).json(self)
    }
}

impl<T: Serialize> Serialize for ApiSuccess<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut resp = serializer.serialize_struct("Success", 2).unwrap();
        resp.serialize_field("error", &false).unwrap();
        resp.serialize_field("data", &self.0).unwrap();
        resp.end()
    }
}

/// A failed API response. Optionally returns a JSON object with more information.
#[derive(Debug)]
pub struct ApiError<T: Serialize + Debug>(pub String, pub Option<T>);

impl<T: Serialize + Debug> ApiError<T> {
    pub fn into_response(self, code: StatusCode) -> HttpResponse {
        HttpResponseBuilder::new(code).json(self)
    }
}

impl<T: Serialize + Debug> Serialize for ApiError<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        if let Some(t) = &self.1 {
            let mut resp = serializer.serialize_struct("Error", 3).unwrap();
            resp.serialize_field("error", &true).unwrap();
            resp.serialize_field("code", &self.0).unwrap();
            resp.serialize_field("data", &t).unwrap();
            resp.end()
        } else {
            let mut resp = serializer.serialize_struct("Error", 2).unwrap();
            resp.serialize_field("error", &true).unwrap();
            resp.serialize_field("code", &self.0).unwrap();
            resp.end()
        }
    }
}
