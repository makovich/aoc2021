#[derive(Debug)]
struct Rdr {
    pos: usize,
    buf: Vec<u8>,
}

impl Rdr {
    fn from(mut msg: &str) -> Self {
        let mut buf = Vec::new();
        loop {
            if msg.len() == 0 {
                break;
            }
            let (byte, rest) = msg.split_at(2);
            buf.push(u8::from_str_radix(byte, 16).unwrap());
            msg = rest;
        }
        buf.extend_from_slice(&[0, 0, 0]); // sentinel
        Self { pos: 0, buf }
    }

    fn read_ver(&mut self) -> u32 {
        let (bits, _) = self.buf[(self.pos / 8)..].split_at(std::mem::size_of::<u16>());
        let mut val = u16::from_be_bytes(bits.try_into().unwrap());
        val <<= self.pos % 8;
        val >>= u16::BITS - 3;
        self.pos += 3;
        val as u32
    }

    fn read_typ(&mut self) -> u8 {
        let (bits, _) = self.buf[(self.pos / 8)..].split_at(std::mem::size_of::<u16>());
        let mut val = u16::from_be_bytes(bits.try_into().unwrap());
        val <<= self.pos % 8;
        val >>= u16::BITS - 3;
        self.pos += 3;
        val as u8
    }

    fn read_lv(&mut self) -> u32 {
        let mut res = 0u32;
        loop {
            let (bits, _) = self.buf[(self.pos / 8)..].split_at(std::mem::size_of::<u16>());
            let mut val = u16::from_be_bytes(bits.try_into().unwrap());
            val <<= self.pos % 8;
            let last = val & 0x8000 == 0;
            val <<= 1;
            val >>= u16::BITS - 4;
            res |= val as u32;
            self.pos += 1 + 4;
            if last {
                break;
            }
            res <<= 4;
        }
        res
    }

    fn read_op_len(&mut self) -> Len {
        let (bits, _) = self.buf[(self.pos / 8)..].split_at(std::mem::size_of::<u32>());
        let mut val = u32::from_be_bytes(bits.try_into().unwrap());
        val <<= self.pos % 8;
        val >>= 16;
        let mut val = val as u16;
        let len_in_bits = val & 0x8000 == 0;
        let len = if len_in_bits { 15 } else { 11 };
        val <<= 1;
        val >>= u16::BITS - len;
        self.pos += 1 + len as usize;
        if len_in_bits {
            Len::Bits(val as u16)
        } else {
            Len::Pkgs(val as u16)
        }
    }

    fn read_pkg(&mut self) -> u32 {
        let mut res = 0;
        let ver = self.read_ver();
        let typ = self.read_typ();
        match typ {
            4 => {
                self.read_lv();
            }
            _ => match self.read_op_len() {
                Len::Pkgs(n) => {
                    for _ in 0..n {
                        res += self.read_pkg();
                    }
                }
                Len::Bits(n) => {
                    let stop_at = self.pos + n as usize;
                    while self.pos < stop_at {
                        res += self.read_pkg();
                    }
                }
            },
        }
        res + ver
    }
}

#[derive(Debug)]
enum Len {
    Bits(u16),
    Pkgs(u16),
}

pub fn main() {
    let mut r = Rdr::from(include_str!("./day16.input"));
    println!("{:?}", r.read_pkg());
}

#[test]
fn h() {
    let mut r = Rdr::from("A0016C880162017C3686B18A3D4780");
    println!("{:?}", r.read_pkg());
}

#[test]
fn g() {
    let mut r = Rdr::from("C0015000016115A2E0802F182340");
    println!("{:?}", r.read_pkg());
}

#[test]
fn f() {
    let mut r = Rdr::from("620080001611562C8802118E34");
    println!("{:?}", r.read_pkg());
}

#[test]
fn e() {
    let mut r = Rdr::from("8A004A801A8002F478");
    println!("{:?}", r.read_pkg());
}

#[test]
fn d() {
    let mut r = Rdr::from("EE00D40C823060");
    println!("{:?}", r.read_pkg());
}

#[test]
fn c() {
    let mut r = Rdr::from("38006F45291200");
    println!("{:?}", r.read_pkg());
}

#[test]
fn b() {
    let mut r = Rdr::from("D2FE28");
    println!("{:?}", r.read_pkg());
}

#[test]
fn a() {
    let mut r = Rdr::from("D2FE28");
    println!("{:?}", r.read_ver());
    println!("{:?}", r.read_typ());
    println!("{:?}", r.read_lv());
}
