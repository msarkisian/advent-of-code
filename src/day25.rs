use std::{fmt::Display, str::FromStr};

struct Snafu(isize);

struct ParseSnafuError;

impl FromStr for Snafu {
    type Err = ParseSnafuError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sum = 0;
        let mut radix = 1;

        for char in s.chars().rev() {
            match char {
                '0' => (),
                '1' => sum += radix,
                '2' => sum += 2 * radix,
                '-' => sum -= radix,
                '=' => sum -= 2 * radix,
                _ => {
                    return Err(ParseSnafuError);
                }
            }
            radix *= 5;
        }
        Ok(Snafu(sum))
    }
}
