//! A usefull way to manipulate bits stream
//! however this is not memory optimized as
//! the bit stream is encoded as a `String`
//! instead of `Vec<bool>`.

use std::fmt::Binary;
use std::mem::size_of;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Endianness {
    BigEndian,
    LittleEndian,
}

/// The structure owning the bit stream
#[derive(Debug)]
pub struct Bits {
    bits: String,
    delimiter: char,
    endianness: Endianness,
}

impl Bits {
    /******************************** CONSTRUCTORS ********************************/
    pub fn from_u8_big_endian(data: &[u8]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:08b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_u8_little_endian(data: &[u8]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:08b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_u16_big_endian(data: &[u16]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:016b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_u16_little_endian(data: &[u16]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:016b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_u32_big_endian(data: &[u32]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:032b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_u32_little_endian(data: &[u32]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:032b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_u64_big_endian(data: &[u64]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:064b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_u64_little_endian(data: &[u64]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:064b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_u128_big_endian(data: &[u128]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:0128b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_u128_little_endian(data: &[u128]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:0128b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_usize_big_endian(data: &[usize]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:064b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_usize_little_endian(data: &[usize]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:064b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_i8_big_endian(data: &[i8]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:08b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_i8_little_endian(data: &[i8]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:08b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_i16_big_endian(data: &[i16]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:016b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_i16_little_endian(data: &[i16]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:016b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_i32_big_endian(data: &[i32]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:032b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_i32_little_endian(data: &[i32]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:032b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_i64_big_endian(data: &[i64]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:064b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_i64_little_endian(data: &[i64]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:064b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_i128_big_endian(data: &[i128]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:0128b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_i128_little_endian(data: &[i128]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:0128b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    pub fn from_isize_big_endian(data: &[isize]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| format!("{}|", format!("{:064b}", *b).chars().collect::<String>()))
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::BigEndian,
        }
    }

    pub fn from_isize_little_endian(data: &[isize]) -> Bits {
        Bits {
            bits: data
                .iter()
                .map(|b| {
                    format!(
                        "{}|",
                        format!("{:064b}", *b).chars().rev().collect::<String>()
                    )
                })
                .collect::<String>(),
            delimiter: '|',
            endianness: Endianness::LittleEndian,
        }
    }

    /******************************** CONSUMERS ********************************/
    /**************** VARIABLE LENGTH ****************/
    /******** UNSIGNED ********/
    pub fn consume_next_data_as_u8(&mut self, size_to_read: usize) -> Result<u8, ParseIntError> {
        let res = self.peek_next_data_as_u8(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_u8_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<u8, ParseIntError> {
        let res = self.peek_next_data_as_u8_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_u16(&mut self, size_to_read: usize) -> Result<u16, ParseIntError> {
        let res = self.peek_next_data_as_u16(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_u16_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<u16, ParseIntError> {
        let res = self.peek_next_data_as_u16_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_u32(&mut self, size_to_read: usize) -> Result<u32, ParseIntError> {
        let res = self.peek_next_data_as_u32(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_u32_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<u32, ParseIntError> {
        let res = self.peek_next_data_as_u32_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_u64(&mut self, size_to_read: usize) -> Result<u64, ParseIntError> {
        let res = self.peek_next_data_as_u64(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_u64_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<u64, ParseIntError> {
        let res = self.peek_next_data_as_u64_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_u128(
        &mut self,
        size_to_read: usize,
    ) -> Result<u128, ParseIntError> {
        let res = self.peek_next_data_as_u128(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_u128_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<u128, ParseIntError> {
        let res = self.peek_next_data_as_u128_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_usize(
        &mut self,
        size_to_read: usize,
    ) -> Result<usize, ParseIntError> {
        let res = self.peek_next_data_as_usize(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_usize_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<usize, ParseIntError> {
        let res = self.peek_next_data_as_usize_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    /******** SIGNED ********/
    pub fn consume_next_data_as_i8(&mut self, size_to_read: usize) -> Result<i8, ParseIntError> {
        let res = self.peek_next_data_as_i8(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_i8_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<i8, ParseIntError> {
        let res = self.peek_next_data_as_i8_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_i16(&mut self, size_to_read: usize) -> Result<i16, ParseIntError> {
        let res = self.peek_next_data_as_i16(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_i16_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<i16, ParseIntError> {
        let res = self.peek_next_data_as_i16_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_i32(&mut self, size_to_read: usize) -> Result<i32, ParseIntError> {
        let res = self.peek_next_data_as_i32(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_i32_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<i32, ParseIntError> {
        let res = self.peek_next_data_as_i32_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_i64(&mut self, size_to_read: usize) -> Result<i64, ParseIntError> {
        let res = self.peek_next_data_as_i64(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_i64_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<i64, ParseIntError> {
        let res = self.peek_next_data_as_i64_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_i128(
        &mut self,
        size_to_read: usize,
    ) -> Result<i128, ParseIntError> {
        let res = self.peek_next_data_as_i128(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_i128_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<i128, ParseIntError> {
        let res = self.peek_next_data_as_i128_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_isize(
        &mut self,
        size_to_read: usize,
    ) -> Result<isize, ParseIntError> {
        let res = self.peek_next_data_as_isize(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    pub fn consume_next_data_as_isize_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<isize, ParseIntError> {
        let res = self.peek_next_data_as_isize_reversed(size_to_read)?;
        self.move_n_bits(size_to_read);
        Ok(res)
    }

    /**************** FIXED LENGTH ****************/
    /******** UNSIGNED ********/
    pub fn consume_next_unsigned_8_bits(&mut self) -> Result<u8, ParseIntError> {
        self.consume_next_data_as_u8(8)
    }

    pub fn consume_next_unsigned_8_bits_reversed(&mut self) -> Result<u8, ParseIntError> {
        self.consume_next_data_as_u8_reversed(8)
    }

    pub fn consume_next_unsigned_16_bits(&mut self) -> Result<u16, ParseIntError> {
        self.consume_next_data_as_u16(16)
    }

    pub fn consume_next_unsigned_16_bits_reversed(&mut self) -> Result<u16, ParseIntError> {
        self.consume_next_data_as_u16_reversed(16)
    }

    pub fn consume_next_unsigned_32_bits(&mut self) -> Result<u32, ParseIntError> {
        self.consume_next_data_as_u32(32)
    }

    pub fn consume_next_unsigned_32_bits_reversed(&mut self) -> Result<u32, ParseIntError> {
        self.consume_next_data_as_u32_reversed(32)
    }

    pub fn consume_next_unsigned_64_bits(&mut self) -> Result<u64, ParseIntError> {
        self.consume_next_data_as_u64(64)
    }

    pub fn consume_next_unsigned_64_bits_reversed(&mut self) -> Result<u64, ParseIntError> {
        self.consume_next_data_as_u64_reversed(64)
    }

    pub fn consume_next_unsigned_128_bits(&mut self) -> Result<u128, ParseIntError> {
        self.consume_next_data_as_u128(128)
    }

    pub fn consume_next_unsigned_128_bits_reversed(&mut self) -> Result<u128, ParseIntError> {
        self.consume_next_data_as_u128_reversed(128)
    }

    /******** SIGNED ********/
    pub fn consume_next_signed_8_bits(&mut self) -> Result<i8, ParseIntError> {
        self.consume_next_data_as_i8(8)
    }

    pub fn consume_next_signed_8_bits_reversed(&mut self) -> Result<i8, ParseIntError> {
        self.consume_next_data_as_i8_reversed(8)
    }

    pub fn consume_next_signed_16_bits(&mut self) -> Result<i16, ParseIntError> {
        self.consume_next_data_as_i16(16)
    }

    pub fn consume_next_signed_16_bits_reversed(&mut self) -> Result<i16, ParseIntError> {
        self.consume_next_data_as_i16_reversed(16)
    }

    pub fn consume_next_signed_32_bits(&mut self) -> Result<i32, ParseIntError> {
        self.consume_next_data_as_i32(32)
    }

    pub fn consume_next_signed_32_bits_reversed(&mut self) -> Result<i32, ParseIntError> {
        self.consume_next_data_as_i32_reversed(32)
    }

    pub fn consume_next_signed_64_bits(&mut self) -> Result<i64, ParseIntError> {
        self.consume_next_data_as_i64(64)
    }

    pub fn consume_next_signed_64_bits_reversed(&mut self) -> Result<i64, ParseIntError> {
        self.consume_next_data_as_i64_reversed(64)
    }

    pub fn consume_next_signed_128_bits(&mut self) -> Result<i128, ParseIntError> {
        self.consume_next_data_as_i128(128)
    }

    pub fn consume_next_signed_128_bits_reversed(&mut self) -> Result<i128, ParseIntError> {
        self.consume_next_data_as_i128_reversed(128)
    }

    /******************************** PEEKERS ********************************/
    /**************** VARIABLE LENGTH ****************/
    /******** UNSIGNED ********/
    pub fn peek_next_data_as_u8(&mut self, size_to_read: usize) -> Result<u8, ParseIntError> {
        assert!(size_to_read <= 8);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        u8::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_u8_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<u8, ParseIntError> {
        assert!(size_to_read <= 8);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        u8::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_u16(&mut self, size_to_read: usize) -> Result<u16, ParseIntError> {
        assert!(size_to_read <= 16);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        u16::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_u16_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<u16, ParseIntError> {
        assert!(size_to_read <= 16);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        u16::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_u32(&mut self, size_to_read: usize) -> Result<u32, ParseIntError> {
        assert!(size_to_read <= 32);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        u32::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_u32_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<u32, ParseIntError> {
        assert!(size_to_read <= 32);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        u32::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_u64(&mut self, size_to_read: usize) -> Result<u64, ParseIntError> {
        assert!(size_to_read <= 64);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        u64::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_u64_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<u64, ParseIntError> {
        assert!(size_to_read <= 64);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        u64::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_u128(&mut self, size_to_read: usize) -> Result<u128, ParseIntError> {
        assert!(size_to_read <= 128);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        u128::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_u128_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<u128, ParseIntError> {
        assert!(size_to_read <= 128);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        u128::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_usize(&mut self, size_to_read: usize) -> Result<usize, ParseIntError> {
        assert!(size_to_read <= 128);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        usize::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_usize_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<usize, ParseIntError> {
        assert!(size_to_read <= 128);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        usize::from_str_radix(&slice_string, 2)
    }

    /******** SIGNED ********/
    pub fn peek_next_data_as_i8(&mut self, size_to_read: usize) -> Result<i8, ParseIntError> {
        assert!(size_to_read <= 8);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        i8::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_i8_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<i8, ParseIntError> {
        assert!(size_to_read <= 8);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        i8::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_i16(&mut self, size_to_read: usize) -> Result<i16, ParseIntError> {
        assert!(size_to_read <= 16);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        i16::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_i16_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<i16, ParseIntError> {
        assert!(size_to_read <= 16);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        i16::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_i32(&mut self, size_to_read: usize) -> Result<i32, ParseIntError> {
        assert!(size_to_read <= 32);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        i32::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_i32_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<i32, ParseIntError> {
        assert!(size_to_read <= 32);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        i32::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_i64(&mut self, size_to_read: usize) -> Result<i64, ParseIntError> {
        assert!(size_to_read <= 64);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        i64::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_i64_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<i64, ParseIntError> {
        assert!(size_to_read <= 64);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        i64::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_i128(&mut self, size_to_read: usize) -> Result<i128, ParseIntError> {
        assert!(size_to_read <= 128);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        i128::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_i128_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<i128, ParseIntError> {
        assert!(size_to_read <= 128);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        i128::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_isize(&mut self, size_to_read: usize) -> Result<isize, ParseIntError> {
        assert!(size_to_read <= 128);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, false);
        isize::from_str_radix(&slice_string, 2)
    }

    pub fn peek_next_data_as_isize_reversed(
        &mut self,
        size_to_read: usize,
    ) -> Result<isize, ParseIntError> {
        assert!(size_to_read <= 128);
        let slice_string = self.get_next_n_bits_as_string(size_to_read, true);
        isize::from_str_radix(&slice_string, 2)
    }

    /******** OTHER ********/
    pub fn peek_next_data_as_string(&mut self, size_to_read: usize) -> String {
        self.get_next_n_bits_as_string(size_to_read, false)
    }

    pub fn peek_next_data_as_string_reversed(&mut self, size_to_read: usize) -> String {
        self.get_next_n_bits_as_string(size_to_read, true)
    }

    /**************** FIXED LENGTH ****************/
    /******** UNSIGNED ********/
    pub fn peek_next_unsigned_8_bits(&mut self) -> Result<u8, ParseIntError> {
        self.peek_next_data_as_u8(8)
    }

    pub fn peek_next_unsigned_8_bits_reversed(&mut self) -> Result<u8, ParseIntError> {
        self.peek_next_data_as_u8_reversed(8)
    }

    pub fn peek_next_unsigned_16_bits(&mut self) -> Result<u16, ParseIntError> {
        self.peek_next_data_as_u16(16)
    }

    pub fn peek_next_unsigned_16_bits_reversed(&mut self) -> Result<u16, ParseIntError> {
        self.peek_next_data_as_u16_reversed(16)
    }

    pub fn peek_next_unsigned_32_bits(&mut self) -> Result<u32, ParseIntError> {
        self.peek_next_data_as_u32(32)
    }

    pub fn peek_next_unsigned_32_bits_reversed(&mut self) -> Result<u32, ParseIntError> {
        self.peek_next_data_as_u32_reversed(32)
    }

    pub fn peek_next_unsigned_64_bits(&mut self) -> Result<u64, ParseIntError> {
        self.peek_next_data_as_u64(64)
    }

    pub fn peek_next_unsigned_64_bits_reversed(&mut self) -> Result<u64, ParseIntError> {
        self.peek_next_data_as_u64_reversed(64)
    }

    pub fn peek_next_unsigned_128_bits(&mut self) -> Result<u128, ParseIntError> {
        self.peek_next_data_as_u128(128)
    }

    pub fn peek_next_unsigned_128_bits_reversed(&mut self) -> Result<u128, ParseIntError> {
        self.peek_next_data_as_u128_reversed(128)
    }

    /******** SIGNED ********/
    pub fn peek_next_signed_8_bits(&mut self) -> Result<i8, ParseIntError> {
        self.peek_next_data_as_i8(8)
    }

    pub fn peek_next_signed_8_bits_reversed(&mut self) -> Result<i8, ParseIntError> {
        self.peek_next_data_as_i8_reversed(8)
    }

    pub fn peek_next_signed_16_bits(&mut self) -> Result<i16, ParseIntError> {
        self.peek_next_data_as_i16(16)
    }

    pub fn peek_next_signed_16_bits_reversed(&mut self) -> Result<i16, ParseIntError> {
        self.peek_next_data_as_i16_reversed(16)
    }

    pub fn peek_next_signed_32_bits(&mut self) -> Result<i32, ParseIntError> {
        self.peek_next_data_as_i32(32)
    }

    pub fn peek_next_signed_32_bits_reversed(&mut self) -> Result<i32, ParseIntError> {
        self.peek_next_data_as_i32_reversed(32)
    }

    pub fn peek_next_signed_64_bits(&mut self) -> Result<i64, ParseIntError> {
        self.peek_next_data_as_i64(64)
    }

    pub fn peek_next_signed_64_bits_reversed(&mut self) -> Result<i64, ParseIntError> {
        self.peek_next_data_as_i64_reversed(64)
    }

    pub fn peek_next_signed_128_bits(&mut self) -> Result<i128, ParseIntError> {
        self.peek_next_data_as_i128(128)
    }

    pub fn peek_next_signed_128_bits_reversed(&mut self) -> Result<i128, ParseIntError> {
        self.peek_next_data_as_i128_reversed(128)
    }

    /******************************** OTHER ********************************/
    pub fn as_vec_bool(&self) -> Vec<bool> {
        self.bits
            .chars()
            .filter(|c| *c != self.delimiter)
            .map(|c| c == '1')
            .collect()
    }

    pub fn transform_as_vec_bool<T>(value: T) -> Vec<bool>
    where
        T: Sized + Binary,
    {
        let size = size_of::<T>() * 8;
        let mut v: Vec<bool> = format!("{:b}", value).chars().map(|c| c == '1').collect();

        while v.len() < size {
            v.insert(0, false);
        }

        v
    }

    /******************************** PRIVATE ********************************/
    fn get_next_n_bits(&mut self, size_to_read: usize) -> Vec<char> {
        assert!(size_to_read <= self.bits.len());
        let mut idx: usize = 0;
        let mut slice: Vec<char> = Vec::new();
        while slice.len() != size_to_read {
            let current = self.bits.chars().nth(idx).unwrap();
            if current != self.delimiter {
                slice.push(current);
            }
            idx += 1;
        }
        slice
    }

    fn get_next_n_bits_as_string(&mut self, size_to_read: usize, reverse: bool) -> String {
        let slice = self.get_next_n_bits(size_to_read);
        if reverse {
            slice.iter().rev().collect::<String>()
        } else {
            slice.iter().collect::<String>()
        }
    }

    fn move_n_bits(&mut self, n: usize) {
        assert!(n < self.bits.len());
        let x = &self.bits[..=n];
        let nb_delim = x.chars().filter(|c| *c == self.delimiter).count();
        self.bits = String::from(&self.bits[n + nb_delim..]);
    }
}
