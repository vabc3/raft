extern crate crypto;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crypto::digest::Digest;
use crypto::sha2::Sha256;

const KEYSIZE: usize = 32;

pub struct Key([u8; KEYSIZE]);

impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for item in self.0.into_iter() {
            try!(write!(f, "{:01$x}", item,2));
        }
        Ok(())
    }
}

impl Key {
    pub fn new(input: &str) -> Self {
        let mut sha = Sha256::new();
        sha.input_str(input);
        // let mut bytes = vec![0u8; KEYSIZE];
        // sha.result(bytes.as_mut_slice());
        let mut bytes = [0;KEYSIZE];
        sha.result(&mut bytes);
        let mut k1 = [0; KEYSIZE];
        k1.clone_from_slice(&bytes);
        Key(k1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
