use serde_json;
use tera;

error_chain!{
    links {
        TemplateError(tera::Error, tera::ErrorKind);
    }

    foreign_links {
       IoError(::std::io::Error);
       JsonError(serde_json::Error);
    }
}
