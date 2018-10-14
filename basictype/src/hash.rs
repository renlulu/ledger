
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

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
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

#[cfg(test)]
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
        h256.0 = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,1u8,1u8,1u8];
        let h256_reversed = h256.reserve();
        assert_eq!(h256_reversed.0,[1u8,1u8,1u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8]);
    }
}
