#[macro_export]
macro_rules! ensure_ascii {
    ($s:expr) => {
        if !$s.is_ascii() {
            return Err(anyhow::anyhow!("ascii required, got {}", $s));
        }
    };
}

#[macro_export]
macro_rules! ensure_len {
    ($val:expr, $expected:expr) => {
        if $val.len() != $expected {
            return Err(anyhow::anyhow!(
                "invalid length: expected {}, got {}",
                $expected,
                $val.len()
            ));
        }
    };
}

#[macro_export]
macro_rules! impl_bitflags_u8 {
    ($t:ty) => {
        impl From<u8> for $t {
            fn from(v: u8) -> Self {
                <$t>::from_bits_truncate(v)
            }
        }

        impl From<$t> for u8 {
            fn from(flags: $t) -> Self {
                flags.bits()
            }
        }
    };
}
