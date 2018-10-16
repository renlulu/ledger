extern crate basictype;
extern crate crypto as mcrypto;

use basictype::hash::Hash256;
use mcrypto::digest::Digest;
use mcrypto::sha2::Sha256;


//这两个方法就是
pub fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);

//    let out: &mut [u8] = &mut []; //都是泪啊
    hasher.result_str()
}

pub fn double_sha256(input: &str) -> String {
    sha256(&sha256(input)[..])
}

//废物


pub fn sha2<'a>(input: &[u8]) -> Hash256 {
    let mut hasher = Sha256::new();
    let mut result = Hash256::default();
    hasher.input(input);
    hasher.result(&mut *result);
    result
}





#[cfg(test)]
mod tests {
    use double_sha256;
    use sha2;
    use sha256;

    #[test]
    fn test_sha2() {
        let slice = b"hello";
        let h256 = sha2(&slice[..]);
        println!("{:?}", h256);
    }


    #[test]
    fn test_sha256() {
        let out = sha256("hello");
        let expected = String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
        println!("{}", out);
        assert_eq!(out, expected)
    }

    #[test]
    fn test_double_sha256() {
        let out = double_sha256("hello");
        let expected = String::from("d7914fe546b684688bb95f4f888a92dfc680603a75f23eb823658031fff766d9");
        println!("{}", out);
        assert_eq!(out, expected)
    }
}
