
struct Decoder {
  data: Vec<u8>,
  size: usize,
  current_position: usize,
}

struct PacketInfo {
  value: u64,
  sum_versions: u64,
}


pub fn run() {
  let data = "020D708041258C0B4C683E61F674A1401595CC3DE669AC4FB7BEFEE840182CDF033401296F44367F938371802D2CC9801A980021304609C431007239C2C860400F7C36B005E446A44662A2805925FF96CBCE0033C5736D13D9CFCDC001C89BF57505799C0D1802D2639801A900021105A3A43C1007A1EC368A72D86130057401782F25B9054B94B003013EDF34133218A00D4A6F1985624B331FE359C354F7EB64A8524027D4DEB785CA00D540010D8E9132270803F1CA1D416200FDAC01697DCEB43D9DC5F6B7239CCA7557200986C013912598FF0BE4DFCC012C0091E7EFFA6E44123CE74624FBA01001328C01C8FF06E0A9803D1FA3343E3007A1641684C600B47DE009024ED7DD9564ED7DD940C017A00AF26654F76B5C62C65295B1B4ED8C1804DD979E2B13A97029CFCB3F1F96F28CE43318560F8400E2CAA5D80270FA1C90099D3D41BE00DD00010B893132108002131662342D91AFCA6330001073EA2E0054BC098804B5C00CC667B79727FF646267FA9E3971C96E71E8C00D911A9C738EC401A6CBEA33BC09B8015697BB7CD746E4A9FD4BB5613004BC01598EEE96EF755149B9A049D80480230C0041E514A51467D226E692801F049F73287F7AC29CB453E4B1FDE1F624100203368B3670200C46E93D13CAD11A6673B63A42600C00021119E304271006A30C3B844200E45F8A306C8037C9CA6FF850B004A459672B5C4E66A80090CC4F31E1D80193E60068801EC056498012804C58011BEC0414A00EF46005880162006800A3460073007B620070801E801073002B2C0055CEE9BC801DC9F5B913587D2C90600E4D93CE1A4DB51007E7399B066802339EEC65F519CF7632FAB900A45398C4A45B401AB8803506A2E4300004262AC13866401434D984CA4490ACA81CC0FB008B93764F9A8AE4F7ABED6B293330D46B7969998021C9EEF67C97BAC122822017C1C9FA0745B930D9C480";
  let mut decoder = decoder_from_hex_string(data);

  let packet_info = decode_packet(&mut decoder);
  println!("sum_versions = {}", packet_info.sum_versions);
  println!("value = {}", packet_info.value);
}


fn decode_packet(decoder: &mut Decoder) -> PacketInfo {
  let version = drain(decoder, 3);
  let type_id = drain(decoder, 3);

  // Info to track
  let mut packet_info = PacketInfo {
    value: 0, 
    sum_versions: version
  };

  if type_id == 4 {
    // Literal Value Packet
    let mut value = 0;
    loop {
      let first_bit = drain(decoder, 1);
      value <<= 4;
      value += drain(decoder, 4);
      if first_bit == 0 {
        break;
      }
    }
    packet_info.value = value;
  }
  else {
    // Operator Packet
    let mut values: Vec<u64> = Vec::new();

    if drain(decoder, 1) == 0 {
      // 15-bit length of sub-packets
      let sub_packet_length = drain(decoder, 15) as usize;

      let starting_decoder_position = decoder.current_position;
      let ending_decoder_position = starting_decoder_position + sub_packet_length;
      loop {
        let sub_packet_info = decode_packet(decoder);

        packet_info.sum_versions += sub_packet_info.sum_versions;
        values.push(sub_packet_info.value);

        if decoder.current_position == ending_decoder_position {
          break;
        }
      }
    }
    else {
      // 11-bit number of sub-packets immediately contained
      let num_sub_packets = drain(decoder, 11);

      for _ in 0..num_sub_packets {
        let sub_packet_info = decode_packet(decoder);

        packet_info.sum_versions += sub_packet_info.sum_versions;
        values.push(sub_packet_info.value);
      }
    }

    // Calculate value from values...
    packet_info.value = match type_id {
      0 => values.iter().sum(),
      1 => values.iter().product(),
      2 => *values.iter().min().unwrap(),
      3 => *values.iter().max().unwrap(),
      5 => {if values[0] > values[1] {1} else {0}}
      6 => {if values[0] < values[1] {1} else {0}}
      7 => {if values[0] == values[1] {1} else {0}}
      _ => {panic!()}
    };
  }

  packet_info
}


// Decoder Implementation ---

fn push_hex(decoder: &mut Decoder, c: char) {

  if decoder.size % 4 != 0 {
    panic!("Not implemented.");
  }
  
  if decoder.current_position != 0 {
    panic!("Not implemented.");
  }
  
  let bits = match c {
    '0' => 0x0, '1' => 0x1, '2' => 0x2, '3' => 0x3, 
    '4' => 0x4, '5' => 0x5, '6' => 0x6, '7' => 0x7, 
    '8' => 0x8, '9' => 0x9, 'A' => 0xA, 'B' => 0xB,
    'C' => 0xC, 'D' => 0xD, 'E' => 0xE, 'F' => 0xF,
    _ => {panic!()}
  };

  let byte_position = decoder.size >> 3;
  if decoder.size % 8 == 0  {
    decoder.data.push(bits << 4);
  }
  else {
    decoder.data[byte_position] |= bits;
  }
  decoder.size += 4;
}

fn decoder_from_hex_string(str: &str) -> Decoder {
  let mut decoder = Decoder {
    data: Vec::new(),
    size: 0,
    current_position: 0,
  };

  for c in str.chars() {
    push_hex(&mut decoder, c);
  }

  decoder
}

fn _decoder_from_binary_string(str: &str) -> Decoder {
  let mut decoder = Decoder {
    data: Vec::new(),
    size: 0,
    current_position: 0,
  };

  for c in str.chars() {
    let bit = match c {
      '0' => 0,
      '1' => 1,
      _ => {panic!()}
    };

    let byte_position = decoder.size >> 3;
    if decoder.data.len() == byte_position {
      decoder.data.push(0);
    }

    let bit_position = 7 - (decoder.size % 8);
    decoder.data[byte_position] |= bit << bit_position;
    decoder.size += 1;
  }

  decoder
}


fn peek(decoder: &Decoder, num_bits: usize) -> u64 {

  if decoder.current_position + num_bits > decoder.size {
    panic!("Not enough bits left.");
  }

  let mut value: u64 = 0;
  let mut bits_left = num_bits;

  let mut current_byte_position = decoder.current_position >> 3;
  let current_bit_position = decoder.current_position - 8 * current_byte_position;

  // First byte...
  let first_byte = decoder.data[current_byte_position];
  let masked_first_byte = first_byte << current_bit_position >> current_bit_position;
  let bits_left_in_byte = 8 - (decoder.current_position % 8);

  if bits_left <= bits_left_in_byte {
    let bits = masked_first_byte >> (bits_left_in_byte - num_bits);
    value += bits as u64;
    bits_left = 0;
  }
  else {
    value += masked_first_byte as u64;
    bits_left -= bits_left_in_byte;
  }
  current_byte_position += 1;

  // Remaining whole bytes...
  while bits_left >= 8 {
    value <<= 8;
    value += decoder.data[current_byte_position] as u64;
    current_byte_position += 1;
    bits_left -= 8;
  }

  // Last byte...
  if bits_left > 0 {
    let bits = decoder.data[current_byte_position] >> (8 - bits_left);
    value <<= bits_left;
    value += bits as u64;
  }

  value
}

fn drain(decoder: &mut Decoder, num_bits: usize) -> u64 {
  let value: u64 = peek(&decoder, num_bits);
  decoder.current_position += num_bits;
  value
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_hex_parsing() {
    let data = "D2FE28";
    let mut decoder = decoder_from_hex_string(data);

    assert!(decoder.size == 24);
    assert!(drain(&mut decoder, 1) == 1);
    assert!(drain(&mut decoder, 1) == 1);
    assert!(drain(&mut decoder, 1) == 0);
    assert!(drain(&mut decoder, 1) == 1);
    assert!(peek(&decoder, 5) == 5);
    assert!(drain(&mut decoder, 5) == 5);
    assert!(drain(&mut decoder, 15) == 32296);
  }

  #[test]
  fn test_simple_binary_parsing() {
    let data = "110100101111111000101000";
    let mut decoder = _decoder_from_binary_string(data);

    assert!(decoder.size == 24);
    assert!(drain(&mut decoder, 1) == 1);
    assert!(drain(&mut decoder, 1) == 1);
    assert!(drain(&mut decoder, 1) == 0);
    assert!(drain(&mut decoder, 1) == 1);
    assert!(peek(&decoder, 5) == 5);
    assert!(drain(&mut decoder, 5) == 5);
    assert!(drain(&mut decoder, 15) == 32296);
  }

  #[test]
  fn test_simple_literal() {
    let data = "110100101111111000101000";
    let mut decoder = _decoder_from_binary_string(data);
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.sum_versions == 6);
  }

  #[test]
  fn test_simple_literal_value() {
    let data = "110100101111111000101000";
    let mut decoder = _decoder_from_binary_string(data);
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.value == 2021);
  }

  #[test]
  fn test_operator_packet_with_sub_length() {
    let data = "38006F45291200";
    let mut decoder = decoder_from_hex_string(data);
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.sum_versions == 1 + 6 + 2);
  }

  #[test]
  fn test_operator_packet_with_sub_count() {
    let data = "EE00D40C823060";
    let mut decoder = decoder_from_hex_string(data);
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.sum_versions == 7 + 2 + 4 + 1);
  }

  #[test] 
  fn test_other_examples_1() {
    let mut decoder = decoder_from_hex_string("8A004A801A8002F478");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.sum_versions == 16);
  }

  #[test] 
  fn test_other_examples_2() {
    let mut decoder = decoder_from_hex_string("620080001611562C8802118E34");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.sum_versions == 12);
  }

  #[test] 
  fn test_other_examples_3() {
    let mut decoder = decoder_from_hex_string("C0015000016115A2E0802F182340");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.sum_versions == 23);
  }

  #[test] 
  fn test_other_examples_4() {
    let mut decoder = decoder_from_hex_string("A0016C880162017C3686B18A3D4780");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.sum_versions == 31);
  }

  #[test] 
  fn test_value_sum() {
    let mut decoder = decoder_from_hex_string("C200B40A82");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.value == 3);
  }

  #[test] 
  fn test_value_product() {
    let mut decoder = decoder_from_hex_string("04005AC33890");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.value == 54);
  }

  #[test] 
  fn test_value_min() {
    let mut decoder = decoder_from_hex_string("880086C3E88112");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.value == 7);
  }

  #[test] 
  fn test_value_max() {
    let mut decoder = decoder_from_hex_string("CE00C43D881120");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.value == 9);
  }

  #[test] 
  fn test_value_lessthan() {
    let mut decoder = decoder_from_hex_string("D8005AC2A8F0");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.value == 1);
  }

  #[test] 
  fn test_value_greaterthan() {
    let mut decoder = decoder_from_hex_string("F600BC2D8F");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.value == 0);
  }

  #[test] 
  fn test_value_equal() {
    let mut decoder = decoder_from_hex_string("9C005AC2F8F0");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.value == 0);
  }

  #[test] 
  fn test_value_nesting() {
    let mut decoder = decoder_from_hex_string("9C0141080250320F1802104A08");
    let packet_info = decode_packet(&mut decoder);
    assert!(packet_info.value == 1);
  }

}
