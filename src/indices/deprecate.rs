use std::fmt;
use std::fmt::Display;
use traits::ElasticIndex;

#[derive(Debug, Deserialize)]
pub struct Deprecate {
    callee: String,
    #[serde(rename = "env.DD_LOCATION")]
    location: String,
    #[serde(rename = "env.DD_ROLE")]
    role: String,
    #[serde(rename = "env.DD_SHOW")]
    show: Option<String>,
    #[serde(rename = "env.DD_SEQ")]
    seq: Option<String>,
    #[serde(rename = "env.DD_SHOT")]
    shot: Option<String>,
    label: String,
    #[serde(rename = "logger.callstack")]
    callstack: String,
    #[serde(rename = "logger.message")]
    message: String,
    #[serde(rename = "logger.user")]
    user: String,
}

impl Display for Deprecate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Take care of level
        let mut level = String::new();
        if let Some(ref s) =self.show {
            level = format!("{}",s);
        }
        if let Some(ref s) = self.seq {
            level = format!("{}.{}",level,s);
        }
        if let Some(ref s) = self.shot {
            level = format!("{}.{}", level, s);
        }

        write!(f, "user: {}\nlevel: {}\nrole: {}\nlocation: {}\n\nmessage:\n{}\n\ncallstack:\n{}\n",
        self.user, level, self.role, self.location, self.message, self.callstack)
    }
}

impl ElasticIndex for Deprecate {
    const NAME: &'static str = "deprecate";
}