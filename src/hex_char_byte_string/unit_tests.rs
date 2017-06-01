use super::*;

#[test]
fn new_accepts_valid_input() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert!(if let Some(HexCharByteString(_)) = HexCharByteString::new(input).ok() { true } else { false });
}

#[test]
fn new_rejects_empty_input() {
    let input = "";
    assert!(if let Some(Error::EmptyValue(_)) = HexCharByteString::new(input).err() { true } else { false });
}

#[test]
fn new_rejects_odd_length_hex_input() {
    let input = "f";
    assert!(if let Some(Error::InvalidSize(_)) = HexCharByteString::new(input).err() { true } else { false });
}

#[test]
fn new_rejects_illegal_hex_digit() {
    let input = "abcdefgh";
    assert!(if let Some(Error::IllegalValue(_)) = HexCharByteString::new(input).err() { true } else { false });
}

#[test]
fn implements_as_str() {
    let input = "c0debeef";
    let expected_result = input.clone();

    assert!(HexCharByteString::new(input).unwrap().as_str() == expected_result);
}

#[test]
fn implements_to_string() {
    let input = "deadc0de";
    let expected_result = input.clone().to_string();

    assert!(HexCharByteString::new(input).unwrap().to_string() == expected_result);
}

#[test]
fn implements_from_trait_for_string() {
    let input = "deadbeef";
    let expected_result = input.clone().to_string();

    assert!(String::from(HexCharByteString::new(input).unwrap()) == expected_result);
}

#[test]
fn hex_digit_char_pair_iterator() {
    let mut input = HexCharByteString::new("1234").unwrap().into_iter();
    let expected_results = [Some((HexChar::new('1').unwrap(), HexChar::new('2').unwrap())),
                            Some((HexChar::new('3').unwrap(), HexChar::new('4').unwrap())), None];

    assert!(input.next() == expected_results[0]);
    assert!(input.next() == expected_results[1]);
    assert!(input.next() == expected_results[2]);
}

#[test]
fn from_byte_buffer_succeeds() {
    let hex_byte_str = HexCharByteString::new("0123456789abcdef").ok().unwrap();
    let input = ByteBuffer::from(hex_byte_str.clone());
    let expected_result = hex_byte_str;

    assert!(HexCharByteString::from(input) == expected_result);
}