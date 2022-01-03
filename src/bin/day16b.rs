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

    fn read_lv(&mut self) -> u64 {
        let mut res = 0u64;
        loop {
            let (bits, _) = self.buf[(self.pos / 8)..].split_at(std::mem::size_of::<u16>());
            let mut val = u16::from_be_bytes(bits.try_into().unwrap());
            val <<= self.pos % 8;
            let last = val & 0x8000 == 0;
            val <<= 1;
            val >>= u16::BITS - 4;
            res |= val as u64;
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

    fn read_pkg(&mut self) -> u64 {
        let _ = self.read_ver();
        let typ = self.read_typ();
        match typ {
            4 => self.read_lv(),
            0 => self.read_op(Op::Sum),
            1 => self.read_op(Op::Prod),
            2 => self.read_op(Op::Min),
            3 => self.read_op(Op::Max),
            5 => self.read_op(Op::Gt),
            6 => self.read_op(Op::Lt),
            7 => self.read_op(Op::Eq),
            _ => unreachable!(),
        }
    }

    fn read_op(&mut self, op: Op) -> u64 {
        let mut vals = Vec::new();

        match self.read_op_len() {
            Len::Pkgs(n) => {
                for _ in 0..n {
                    vals.push(self.read_pkg());
                }
            }
            Len::Bits(n) => {
                let stop_at = self.pos + n as usize;
                while self.pos < stop_at {
                    vals.push(self.read_pkg());
                }
            }
        }

        let val = match op {
            Op::Sum => vals.iter().sum(),
            Op::Prod => vals.iter().product(),
            Op::Min => *vals.iter().min().unwrap(),
            Op::Max => *vals.iter().max().unwrap(),
            Op::Gt => (vals[0] > vals[1]) as u64,
            Op::Lt => (vals[0] < vals[1]) as u64,
            Op::Eq => (vals[0] == vals[1]) as u64,
        };

        val
    }
}

#[derive(Debug)]
enum Op {
    Sum,
    Prod,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
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

pub fn complex() {
    let mut r = Rdr::from("9C0141080250320F1802104A08");
    println!("{:?}", r.read_pkg());
}

pub fn ne() {
    let mut r = Rdr::from("9C005AC2F8F0");
    println!("{:?}", r.read_pkg());
}

pub fn gt() {
    let mut r = Rdr::from("F600BC2D8F");
    println!("{:?}", r.read_pkg());
}

pub fn lt() {
    let mut r = Rdr::from("D8005AC2A8F0");
    println!("{:?}", r.read_pkg());
}

pub fn max() {
    let mut r = Rdr::from("CE00C43D881120");
    println!("{:?}", r.read_pkg());
}

pub fn min() {
    let mut r = Rdr::from("880086C3E88112");
    println!("{:?}", r.read_pkg());
}

pub fn product() {
    let mut r = Rdr::from("04005AC33890");
    println!("{:?}", r.read_pkg());
}

pub fn sum() {
    let mut r = Rdr::from("C200B40A82");
    println!("{:?}", r.read_pkg());
}
