error_chain! {
    foreign_links {
        FromUtf8Error(::std::string::FromUtf8Error);
        DecodeError(::data_encoding::DecodeError);
        CeleryError(::celery::error::CeleryError);
    }
}