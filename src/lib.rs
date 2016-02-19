extern crate libc;
use libc::c_int;

extern {
	fn haraka256256(hash: &mut [u8; 32], msg: &[u8; 32]) -> c_int;
	fn haraka512256(hash: &mut [u8; 32], msg: &[u8; 64]) -> c_int;
}

fn print_64(msg: &[u8; 64]) {
    for i in 0..64 {
        print!("{:01$x} ", msg[i], 2);
    }
}

fn print_32(msg: &[u8; 32]) {
    for i in 0..32 {
        print!("{:01$x} ", msg[i], 2);
    }
}

#[no_mangle]
/// Reference hash for msg = [1, 2, 3, ... 31]:
/// "cb b4 e2 a4 a7 b4 71 c6 cc 44 8c d2 64 d1 08 3e af 8f 75 d0 d4 0a c4 f9 73 da 6d 53 bf c0 5c c0"
pub fn haraka256(hash: &mut [u8; 32], msg: [u8; 32]) {
    print_32(&msg);

	unsafe {
		haraka256256(hash, &msg);
	}

    print_32(hash);
}

#[no_mangle]
/// Reference hash for msg = [1, 2, 3, ... 63]:
/// "2f 62 a3 1b e6 eb 3a 1c d6 43 fe a5 e8 69 ff 7c be 86 3b 26 5d b4 b0 0e 7c f3 e3 19 bf a1 66 c1"
pub fn haraka512(hash: &mut [u8; 32], msg: [u8; 64]) {
    print_64(&msg);

	unsafe {
		haraka512256(hash, &msg);
	}

    print_32(hash);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haraka_256() {
        let mut msg: [u8; 32] = [0; 32];
        let mut hash: [u8; 32] = [0; 32];
        let hash_ref_256 = vec!("cb", "b4", "e2", "a4", "a7", "b4", "71", "c6", "cc", "44", "8c",
                                "d2", "64", "d1", "08", "3e", "af", "8f", "75", "d0", "d4", "0a",
                                "c4", "f9", "73", "da", "6d", "53", "bf", "c0", "5c", "c0");

        for i in 0..32 {
            msg[i] = i as u8;
        }

        haraka256(&mut hash, msg);

        for i in 0..32 {
            assert_eq!(format!("{:01$x}", hash[i], 2), hash_ref_256[i]);
        }
    }

    #[test]
    fn test_haraka_512() {
        let mut msg: [u8; 64] = [0; 64];
        let mut hash: [u8; 32] = [0; 32];
        let hash_ref_512 = vec!("2f", "62", "a3", "1b", "e6", "eb", "3a", "1c", "d6", "43", "fe",
                                "a5", "e8", "69", "ff", "7c", "be", "86", "3b", "26", "5d", "b4",
                                "b0", "0e", "7c", "f3", "e3", "19", "bf", "a1", "66", "c1");

        for i in 0..64 {
            msg[i] = i as u8;
        }

        haraka512(&mut hash, msg);

        for i in 0..32 {
            assert_eq!(format!("{:01$x}", hash[i], 2), hash_ref_512[i]);
        }
    }
}
