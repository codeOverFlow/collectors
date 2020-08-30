use collectors::Bits;

fn main() {
    let u8_vec: Vec<u8> = vec![0, 1, 2, 3];
    let u8_arr: [u8; 4] = [0, 1, 2, 3];
    let bits = Bits::from_u8_big_endian(&u8_vec);
    let bits = Bits::from_u8_big_endian(&u8_arr);
}
