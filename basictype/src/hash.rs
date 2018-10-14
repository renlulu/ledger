
macro_rules! impl_hash {
    ($name: ident, $size: expr) => {

        pub struct $name([u8;$size]); //这里是 slice 的写法

        impl Default for $name {
            fn default() -> Self{
                $name([0u8;$size])
            }
        }

        impl AsRef<$name> for $name {
            fn as_ref(&self) -> &$name {
                self
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                let mut c = Self::default();
                c.0.copy_from_slice(&self.0);
                c
            }
        }
    }
}

impl_hash!(H256, 32);

#[cfg(test)]
mod tests {
    use super::H256;

    #[test]
    fn test_default() {
        let h256 = H256::default();
        assert_eq!(h256.0.len(),32);
    }
}
