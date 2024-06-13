use std::str::FromStr;
use uuid::Uuid;

type Format = (&'static str, fn(&str) -> bool);
pub const FORMATS: [Format; 3] = [
    ("uuid", uuid),
    ("ru-inn-individual", ru_inn_individual),
    ("ru-inn-legal-entity", ru_inn_legal_entity),
];

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

fn ru_inn_legal_entity(r: &str) -> bool {
    if r.len() != 10 || r.starts_with("00") {
        return false;
    }

    let coefficients = [2, 4, 10, 3, 5, 9, 4, 6, 8];

    let mut checksum = 0;

    for (i, ch) in r.chars().take(9).enumerate() {
        if let Some(num) = ch.to_digit(10) {
            checksum += num * coefficients[i];
        } else {
            return false;
        }
    }

    r.chars().nth(9).unwrap().to_digit(10) == Some(checksum % 11 % 10)
}

#[cfg(test)]
mod tests {
    use crate::formats::{ru_inn_individual, uuid};

    use super::ru_inn_legal_entity;

    #[test]
    fn valid_uuid() {
        assert!(uuid("539e73de-a2a8-4e8f-907d-0d6c1139bbbf"));
    }

    #[test]
    fn invalid_uuid() {
        assert!(!uuid("539e73deAAA-a2a8-4e8f-907d-2139bbbf"));
    }

    fn check_inn(inn_list: &[&str], is_individual: bool, expected: bool) {
        for inn in inn_list {
            if is_individual {
                assert_eq!(
                    ru_inn_individual(inn),
                    expected,
                    "Valid INN individual test failed for: {inn}"
                );
            } else {
                assert_eq!(
                    ru_inn_legal_entity(inn),
                    expected,
                    "Valid INN legal entity test failed for: {inn}"
                );
            }
        }
    }

    #[test]
    fn invalid_inn_individual() {
        check_inn(
            &[
                "123",
                "123123123123",
                "589522892248",
                "793532525660",
                "617949621244",
                "82397966847",
                "AAAAAAAAAAA",
                "000000000000",
            ],
            true,
            false,
        );
    }

    #[test]
    fn valid_inn_individual() {
        check_inn(
            &[
                "197715976499",
                "101514237669",
                "589522894248",
                "793332525660",
                "617049621244",
                "803197966847",
                "088637982022",
                "620147653223",
                "355447087280",
                "576389730766",
                "442441540930",
                "876208969909",
                "640197925700",
            ],
            true,
            true,
        );
    }

    #[test]
    fn invalid_inn_legal_entity() {
        check_inn(
            &[
                "123",
                "351A9150290",
                "4069583041",
                "9999999999",
                "0000000000",
                "ABCDEFGSDE",
            ],
            false,
            false,
        );
    }

    #[test]
    fn valid_inn_legal_entity() {
        check_inn(
            &[
                "6830692790",
                "7406096779",
                "0293525855",
                "2594070450",
                "5090465656",
                "7256456123",
                "8533645462",
                "8092765562",
                "0356662222",
                "2827189807",
                "8346948900",
                "8976890865",
                "6616124526",
            ],
            false,
            true,
        );
    }
}
