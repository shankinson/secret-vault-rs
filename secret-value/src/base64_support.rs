use crate::SecretValue;
use zeroize::{Zeroize, Zeroizing};

impl SecretValue {
    pub fn as_sensitive_base64_str(&self) -> Zeroizing<String> {
        base64::encode(self.as_sensitive_bytes()).into()
    }

    pub fn from_base64_str(mut hex_string: String) -> Result<Self, base64::DecodeError> {
        let result = Self::new(base64::decode(&hex_string)?);
        hex_string.zeroize();
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    fn generate_secret_value() -> BoxedStrategy<SecretValue> {
        ".*".prop_map(|(mock_bytes)| SecretValue::new(mock_bytes.into()))
            .boxed()
    }

    proptest! {
        #[test]
        fn test_base64_encode(secret_value in generate_secret_value()) {
            let base64_str = secret_value.as_sensitive_base64_str();
            assert_eq!(secret_value.as_sensitive_bytes(), base64::decode(base64_str).unwrap())
        }

        #[test]
        fn test_base64_decode(mock_str in ".*") {
            let base64_str = base64::encode(mock_str.as_bytes());
            let secret_value = SecretValue::from_base64_str(base64_str).unwrap();
            assert_eq!(secret_value.as_sensitive_bytes(), mock_str.as_bytes())
        }
    }
}
