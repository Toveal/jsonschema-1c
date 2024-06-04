use std::str::FromStr;
use uuid::Uuid;

type Format = (&'static str, fn(&str) -> bool);
pub const FORMATS: [Format; 1] = [("uuid", uuid)];

fn uuid(r: &str) -> bool {
    Uuid::from_str(r).is_ok()
}
