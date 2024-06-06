use std::str::FromStr;
use uuid::Uuid;

type Format = (&'static str, fn(&str) -> bool);
pub const FORMATS: [Format; 2] = [("uuid", uuid), ("ru-inn-individual", ru_inn_individual)];

fn uuid(r: &str) -> bool {
    Uuid::from_str(r).is_ok()
}

fn ru_inn_individual(r: &str) -> bool {
    if r.len() != 12 || r.starts_with("00") {
        return false;
    }

    let coefficients1 = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8, 0];
    let coefficients2 = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];

    let mut checksum1 = 0;
    let mut checksum2 = 0;

    for (i, ch) in r.chars().take(11).enumerate() {
        if let Some(num) = ch.to_digit(10) {
            checksum1 += num * coefficients1[i];
            checksum2 += num * coefficients2[i];
        } else {
            return false;
        }
    }

    let n11 = (checksum1 % 11) % 10;
    let n12 = (checksum2 % 11) % 10;
    r.chars().nth(10).unwrap().to_digit(10) == Some(n11)
        && r.chars().nth(11).unwrap().to_digit(10) == Some(n12)
}

