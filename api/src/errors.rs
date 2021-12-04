error_chain! {
    foreign_links {
        FromUtf8Error(::std::string::FromUtf8Error);
        DecodeError(::data_encoding::DecodeError);
        CeleryError(::celery::error::CeleryError);
        ParseIntError(::std::num::ParseIntError);
        BollardError(::bollard::errors::Error);
        IoError(::std::io::Error);
        SqlxError(::sqlx::Error);
        SystemTimeError(::std::time::SystemTimeError);
    }
}

pub struct ErrorResponse {
    status: rocket::http::Status,
    message: String,
}

impl From<Error> for ErrorResponse {
    fn from(error: Error) -> Self {
        ErrorResponse {
            status: rocket::http::Status::InternalServerError,
            message: format!("{}", error)
        }
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::response::Response::build()
            .sized_body(self.message.len(), std::io::Cursor::new(self.message))
            .header(rocket::http::ContentType::new("application", "json"))
            .status(self.status)
            .ok()
    }
}
