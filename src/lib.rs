use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;
use sha2::{Digest, Sha256};

mod rand;

fn hash_code(data: impl AsRef<[u8]>) -> u64 {
    const INITIAL: u64 = 17;

    let mut result = INITIAL;
    for &byte in data.as_ref() {
        let temp = result;
        for _ in 1..37 {
            result = result.wrapping_add(temp);
        }
        result += byte as u64;
    }
    result & i32::MAX as u64
}

fn encode_with_seed<T: AsMut<[u8]>>(mut data: T, seed: u64) -> String {
    const ENCODE_CHARS: &[u8] = b"fUi7oEd)IyZcPQlzHDnARm5thFwJKqjgrX2b8VWaOCY9pM!e3TsvkBxNu614LS0G";

	let data = data.as_mut();
	let rng = Rng::with_seed(seed);
	rng.shuffle(data);

    let mut result = String::new();
    for i in (0..data.len() * 8).step_by(6) {
        let mut lsb = 7 & i;
        let mut real_index = i >> 3;
        let byte = data[real_index];
        if lsb == 0 {
            result.push(ENCODE_CHARS[(63 & byte) as usize].into());
        } else {
            let mut f = byte >> lsb;
            real_index += 1;
            if real_index < data.len() {
                lsb = 8 - lsb;

                f |= data[real_index] << lsb;
            }
            result.push(ENCODE_CHARS[(63 & f) as usize].into())
        }
    }

    result
}

pub fn hash_password<T: AsRef<[u8]>>(email: T, password: T) -> String {
    const SALT: &[u8] = b"helloSTEPN";
	let email_hash = hash_code(&email);

    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
	let hash = {
		let mut hasher = Sha256::new();
		hasher.update(password.as_ref());
		hasher.update(SALT);
		hasher.finalize()
	};
    let hash = hex::encode(hash);

	let data = {
		let mut data: Vec<u8> = Vec::new();
		data.extend_from_slice(hash.as_bytes());
		data.push(b'_');
		data.extend_from_slice(since_the_epoch.as_millis().to_string().as_bytes());
		data
	};

    encode_with_seed(data, email_hash)
}

#[cfg(test)]
mod tests {
    use crate::{encode_with_seed, hash_code, hash_password};

    #[test]
    fn hash_code_test() {
        let result = hash_code("fghfgh@ggg.ggf");
        assert_eq!(result, 1997399150);
    }

    #[test]
    fn encode_with_seed_test() {
        let mut data =
            b"dfb488dff049a35ae6bd81f32888de972edb0a98b47fd68b321ab79bf32c5ee0_1657992108586"
                .to_vec();
        let encoded = encode_with_seed(data.as_mut_slice(), 1997399150);
        assert_eq!(encoded, "XE5FsPWP8FdP3HvQVVvP2E7lxhdFx87F3r7lsIWhuHbPuH5PTIdlNLmQ2y5lWQAlB8bF2F5P8qbPNPAlurAhVm5QxoWFurbQVUvPBH7F");
    }

	#[test]
	fn hash_password_test() {
		let hashed = hash_password("yakof12530@satedly.com", "123456");
		println!("{hashed}");
	}
}
