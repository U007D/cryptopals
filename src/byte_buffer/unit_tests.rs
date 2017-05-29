use super::*;

#[test]
fn from_hex_byte_string_succeeds() {
    let input = HexByteString::new("00112233445566778899aabbccddeeff").unwrap();
    let expected_result = vec![0x00u8, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
                               0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];

    assert!(ByteBuffer::from(input).as_byte_vec() == expected_result);
}
