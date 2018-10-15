
use std::{str};
use rustc_hex::{FromHexError,FromHex};

macro_rules! impl_hash {
    ($name: ident, $size: expr) => {

        #[derive(Debug)]
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

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl str::FromStr for $name {
            type Err = FromHexError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let bytes: Vec<u8> = s.from_hex().unwrap();
                match bytes.len() {
                    $size => {
                        let mut r = [0u8;$size];
                        r.copy_from_slice(&bytes);
                        Ok($name(r))
                    },
                    _ => {
                        Err(FromHexError::InvalidHexLength)

                    }
                }
            }
        }


        impl $name {
            pub fn take(self) -> [u8;$size] {
                self.0
            }

            pub fn size() -> usize {
                $size
            }

            pub fn reserve(&self) -> Self {
                let mut r = self.clone();
                r.0.reverse();
                r
            }
        }
    }
}

impl_hash!(Hash256, 32);

impl Hash256 {

}

#[cfg(test)] //cargo test -- --nocapture
mod tests {
    use super::Hash256;

    #[test]
    fn test_default() {
        let h256 = Hash256::default();
        assert_eq!(h256.0.len(),32);
    }

    #[test]
    fn test_reversed() {
        let mut h256 = Hash256::default();
        let mut h256_pre_reversed = Hash256::default();
        h256.0 = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,1u8,1u8,1u8];
        h256_pre_reversed.0 = [1u8,1u8,1u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
        let h256_reversed = h256.reserve();
        println!("{:?}", h256_reversed);
        println!("{:?}", h256_pre_reversed);
        assert_eq!(h256_reversed,h256_pre_reversed);
    }
}
