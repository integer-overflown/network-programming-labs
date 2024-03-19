pub struct Config {
    reference: u64,
}

impl Config {
    pub const fn new(reference: u64) -> Self {
        Self { reference }
    }

    pub const fn get_number(&self, actual: u64) -> u64 {
        if actual > self.reference {
            self.reference
        } else {
            actual
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_get_number_body((reference, actual, expected): (u64, u64, u64)) -> Result<(), String> {
        let config = Config::new(reference);
        let res = config.get_number(actual);

        if res == expected {
            Ok(())
        } else {
            Err(format!("expected {expected}, got {res}"))
        }
    }

    #[test]
    fn test_get_number() -> Result<(), String> {
        [(6, 8, 6), (6, 6, 6), (2, 1, 1)]
            .into_iter()
            .try_for_each(test_get_number_body)
    }
}
