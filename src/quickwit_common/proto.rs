use std::convert::Infallible;

#[derive(Clone, Copy)]
pub enum ServiceErrorCode {
    // BadRequest,
    Internal,
    // MethodNotAllowed,
    // NotFound,
    // RateLimited,
    // Unavailable,
    // UnsupportedMediaType,
    // NotSupportedYet, //< Used for API that is available in elasticsearch but is not yet available in Quickwit.
}

impl ServiceErrorCode {
    // pub fn to_grpc_status_code(self) -> tonic::Code {
    //     match self {
    //         // ServiceErrorCode::BadRequest => tonic::Code::InvalidArgument,
    //         ServiceErrorCode::Internal => tonic::Code::Internal,
    //         // ServiceErrorCode::MethodNotAllowed => tonic::Code::InvalidArgument,
    //         // ServiceErrorCode::NotFound => tonic::Code::NotFound,
    //         // ServiceErrorCode::RateLimited => tonic::Code::ResourceExhausted,
    //         // ServiceErrorCode::Unavailable => tonic::Code::Unavailable,
    //         // ServiceErrorCode::UnsupportedMediaType => tonic::Code::InvalidArgument,
    //         // ServiceErrorCode::NotSupportedYet => tonic::Code::Unimplemented,
    //     }
    // }
    pub fn to_http_status_code(self) -> http::StatusCode {
        match self {
            // ServiceErrorCode::BadRequest => http::StatusCode::BAD_REQUEST,
            ServiceErrorCode::Internal => http::StatusCode::INTERNAL_SERVER_ERROR,
            // ServiceErrorCode::MethodNotAllowed => http::StatusCode::METHOD_NOT_ALLOWED,
            // ServiceErrorCode::NotFound => http::StatusCode::NOT_FOUND,
            // ServiceErrorCode::RateLimited => http::StatusCode::TOO_MANY_REQUESTS,
            // ServiceErrorCode::Unavailable => http::StatusCode::SERVICE_UNAVAILABLE,
            // ServiceErrorCode::UnsupportedMediaType => http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
            // ServiceErrorCode::NotSupportedYet => http::StatusCode::NOT_IMPLEMENTED,
        }
    }
}

pub trait ServiceError: ToString {
    // fn grpc_error(&self) -> tonic::Status {
    //     let grpc_code = self.status_code().to_grpc_status_code();
    //     let error_msg = self.to_string();
    //     tonic::Status::new(grpc_code, error_msg)
    // }

    fn status_code(&self) -> ServiceErrorCode;
}

impl ServiceError for Infallible {
    fn status_code(&self) -> ServiceErrorCode {
        unreachable!()
    }
}
