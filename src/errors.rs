use serde;
use std::fmt;
use std;



use hyper::error::Error as HyperError;
use serde_json::error::Error as SerdeJsonError;
use std::io::Error as IoError;
use self::serde::de;
use self::serde::Deserializer;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SlackErrorType {
    ChannelNotFound,
    InvalidTsLatest,
    InvalidTsOldest,
    NotAuthed,
    InvalidAuth,
    AccountInactive,
    InvalidArgName,
    UnknownError,
}

struct SlackErrorTypeVisitor;

impl de::Visitor for SlackErrorTypeVisitor {
    type Value = SlackErrorType;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A Slack error type string like `channel_not_found`")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
        where E: de::Error
    {
        match value {
            "channel_not_found" => Ok(SlackErrorType::ChannelNotFound),
            "invalid_ts_latest" => Ok(SlackErrorType::InvalidTsLatest),
            _ => Err(serde::de::Error::custom(format!("Undefined Slack error: {}", value))),   
        }
    }
}

fn deserialize_slack_error_type<D>(deserializer: D) -> std::result::Result<SlackErrorType, D::Error>
    where D: Deserializer
{
    deserializer.deserialize(SlackErrorTypeVisitor)
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackErrorMessage {
    pub ok: bool,
    #[serde(deserialize_with="deserialize_slack_error_type")]
    pub error: SlackErrorType,
}



error_chain!{
    errors {
        /// A representation of a Slack API error as a Rust type
        Slack(s: SlackErrorType) {}
        /// Errors while serializing/deserializing data
        Serialization(msg: String) {}
    }
    foreign_links {
        // Miscellaneous error reported from hyper
        Hyper(HyperError);
        // Error while deserializing JSON
        Json(SerdeJsonError);
        // I/O Errors
        Io(IoError);
    }
}