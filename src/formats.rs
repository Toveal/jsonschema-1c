type Format = (&'static str, fn(&str) -> bool);
pub const FORMATS: [Format; 4] = [
    ("ru-inn-individual", ru_inn_individual),
    ("ru-inn-legal-entity", ru_inn_legal_entity),
    ("kz-iin", kz_iin),
    ("local-date-time", local_date_time),
];

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

fn kz_iin(r: &str) -> bool {
    let first_symbol = r.chars().next();

    if r.len() != 12 || r.chars().all(|ch| first_symbol == Some(ch)) {
        return false;
    }

    let mut checksum = 0;

    for (i, ch) in r.chars().take(11).enumerate() {
        if let Some(num) = ch.to_digit(10) {
            checksum += (u32::try_from(i).unwrap() + 1) * num;
        } else {
            return false;
        }
    }

    let mut control_value = checksum % 11;

    if control_value == 10 {
        checksum = 0;
        for (i, ch) in r.chars().take(11).enumerate() {
            if let Some(num) = ch.to_digit(10) {
                let mut t = (u32::try_from(i).unwrap() + 3) % 11;
                if t == 0 {
                    t = 11;
                }
                checksum += t * num;
            } else {
                return false;
            }
        }
        control_value = checksum % 11;
        if control_value == 10 {
            return false;
        }
    }

    r.chars().nth(11).unwrap().to_digit(10) == Some(control_value)
}

fn local_date_time(datetime: &str) -> bool {
    // Find the position of 'T' or 't' separator
    let Some(t_pos) = datetime.bytes().position(|b| b == b'T' || b == b't') else {
        return false;
    };

    // Split the string into date and time parts
    let (date_part, time_part) = datetime.split_at(t_pos);

    is_valid_date(date_part) && is_valid_time(&time_part[1..])
}

fn is_valid_date(date: &str) -> bool {
    if date.len() != 10 {
        return false;
    }

    let bytes = date.as_bytes();

    // Check format: YYYY-MM-DD
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }

    // Parse year (YYYY)
    let Some(year) = parse_four_digits(&bytes[0..4]) else {
        return false;
    };

    // Parse month (MM)
    let Some(month) = parse_two_digits(&bytes[5..7]) else {
        return false;
    };
    if !(1..=12).contains(&month) {
        return false;
    }

    // Parse day (DD)
    let Some(day) = parse_two_digits(&bytes[8..10]) else {
        return false;
    };
    if day == 0 {
        return false;
    }

    // Check day validity
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day <= 31,
        4 | 6 | 9 | 11 => day <= 30,
        2 => {
            if is_leap_year(year) {
                day <= 29
            } else {
                day <= 28
            }
        }
        _ => unreachable!("Month value is checked above"),
    }
}

#[inline]
fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn is_valid_time(time: &str) -> bool {
    let bytes = time.as_bytes();
    let len = bytes.len();

    if len < 8 {
        // Minimum valid time is "HH:MM:SS"
        return false;
    }

    // Check HH:MM:SS format
    if bytes[2] != b':' || bytes[5] != b':' {
        return false;
    }

    // Parse hour (HH)
    let Some(hour) = parse_two_digits(&bytes[..2]) else {
        return false;
    };

    // Parse minute (MM)
    let Some(minute) = parse_two_digits(&bytes[3..5]) else {
        return false;
    };

    // Parse second (SS)
    let Some(second) = parse_two_digits(&bytes[6..8]) else {
        return false;
    };

    if hour > 23 || minute > 59 || second > 59 {
        return false;
    }

    let mut i = 8;

    // Check fractional seconds (optional)
    if i < len && bytes[i] == b'.' {
        i += 1;
        let mut has_digit = false;
        while i < len && bytes[i].is_ascii_digit() {
            has_digit = true;
            i += 1;
        }
        if !has_digit {
            return false;
        }
    }

    // For local datetime, we've consumed everything
    i == len
}

#[inline]
fn parse_two_digits(bytes: &[u8]) -> Option<u8> {
    let value = u16::from_ne_bytes([bytes[0], bytes[1]]);

    // Check if both bytes are ASCII digits
    if value.wrapping_sub(0x3030) & 0xF0F0 == 0 {
        Some(((value & 0x0F0F).wrapping_mul(2561) >> 8) as u8)
    } else {
        None
    }
}

#[inline]
fn parse_four_digits(bytes: &[u8]) -> Option<u16> {
    let value = u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

    // Check if all bytes are ASCII digits
    if value.wrapping_sub(0x3030_3030) & 0xF0F0_F0F0 == 0 {
        let val = (value & 0x0F0F_0F0F).wrapping_mul(2561) >> 8;
        Some(((val & 0x00FF_00FF).wrapping_mul(6_553_601) >> 16) as u16)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::formats::{kz_iin, ru_inn_individual};

    use super::ru_inn_legal_entity;

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

    #[test]
    fn valid_kz_iin() {
        let inn = [
            "181228500010",
            "730703400015",
            "170624600015",
            "910701300010",
            "840101400014",
            "730812300016",
            "150109600011",
        ];

        for el in inn {
            assert!(kz_iin(el));
        }
    }

    #[test]
    fn invalid_kz_iin() {
        let inn = ["123", "842101400014", "150105600011"];
        for el in inn {
            assert!(!kz_iin(el));
        }
    }

    #[test]
    fn local_date_time_valid() {
        let datetime_list = [
            "2023-10-05T14:30:00",
            "2000-02-29T23:59:59.999",
            "1999-12-31T00:00:00",
            "2024-02-29T12:00:00", // Leap year
        ];

        for datetime in datetime_list {
            assert!(
                super::local_date_time(datetime),
                "Valid local date-time test failed for: {datetime}"
            );
        }
    }
}
