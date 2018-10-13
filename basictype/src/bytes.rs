#[derive(Default,Debug,PartialEq)]
pub struct Bytes{
    bytes: Vec<u8>,
}

impl Bytes {
    pub fn new() -> Self {
        Bytes::default() //关联方法
    }

    pub fn new_with_length(mut length: usize) -> Self {
        let mut vec = Vec::new();
        while length != 0 {
            vec.push(0);
            length = length -1;
        }
        Bytes {
            bytes: vec
        }

    }
}

#[cfg(test)]
mod tests {
    use super::Bytes;

    #[test]
    fn test_new() {
        let b = Bytes::new();
        assert_eq!(b,Bytes {
            bytes: Vec::new()
        })
    }
}
