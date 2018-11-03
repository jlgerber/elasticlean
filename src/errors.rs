
#[derive(Fail, Debug,PartialEq,Eq, PartialOrd, Ord)]
pub enum EcError {
    #[fail(display = "failed to parse: {}", _0)]
    ParseError(String),
    #[fail(display = "reqwest get error: {}", _0)]
    ReqwestGetError(String),
    #[fail(display = "reqwest json error. unable to deserialize to json: {}", _0)]
    ReqwestJsonError(String),
    #[fail(display = "NotImplemented")]
    NotImplemented,
}