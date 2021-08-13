error_chain! {
    foreign_links {
        FromUtf8Error(::std::string::FromUtf8Error);
        DecodeError(::data_encoding::DecodeError);
        CeleryError(::celery::error::CeleryError);
        ParseIntError(::std::num::ParseIntError);
        BollardError(::bollard::errors::Error);
        IoError(::std::io::Error);
        SqlxError(::sqlx::Error);
    }
}
