extern crate basictype;
extern crate crypto;

use basictype::hash::Hash256;
use crypto::digest::Digest;
use crypto::sha2::Sha256;


pub fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);

//    let out: &mut [u8] = &mut []; //都是泪啊
    let out = hasher.result_str();
    out
}


#[cfg(test)]
mod tests {
    use sha256;

    #[test]
    fn test_sha256() {
        let out = sha256("hello");
        let expected = String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
        println!("{}", out);
        assert_eq!(out, expected)
    }
}
