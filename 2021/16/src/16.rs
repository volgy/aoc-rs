aoc::parts!(1, 2);

struct BitStream {
    bits: Vec<u8>,
    pos: usize,
}

impl BitStream {
    fn new<T: Into<Vec<u8>>>(bits: T) -> Self {
        Self {
            bits: bits.into(),
            pos: 0,
        }
    }

    fn field(&mut self, size: usize) -> Option<u64> {
        if self.pos + size > self.bits.len() {
            return None;
        }
        let value = (0..size).fold(0, |acc, i| (acc << 1) | (self.bits[self.pos + i] as u64));
        self.pos += size;
        Some(value)
    }

    fn sub_stream(&mut self, size: usize) -> Option<BitStream> {
        if self.pos + size > self.bits.len() {
            return None;
        }
        let sub_bits = self.bits[self.pos..self.pos + size].to_vec();
        self.pos += size;
        Some(BitStream::new(sub_bits))
    }
}

impl From<&str> for BitStream {
    fn from(s: &str) -> Self {
        let bits: Vec<_> = s
            .chars()
            .flat_map(|c| {
                let n = c.to_digit(16).unwrap();
                (0..4).rev().map(move |i| ((n >> i) & 1) as u8)
            })
            .collect();
        BitStream::new(bits)
    }
}

enum Packet {
    Literal {
        version: u64,
        value: u64,
    },
    Operator {
        version: u64,
        sub_packets: Vec<Packet>,
        opcode: u64,
    },
}

impl Packet {
    fn from_bitstream(bits: &mut BitStream) -> Option<Self> {
        let version = bits.field(3)?;
        let packet_type = bits.field(3)?;
        match packet_type {
            4 => {
                let mut value = 0;
                loop {
                    let group = bits.field(1)?;
                    value = (value << 4) + bits.field(4)?;
                    if group == 0 {
                        break Some(Self::Literal { version, value });
                    }
                }
            }
            opcode => {
                let length_type = bits.field(1)?;
                match length_type {
                    0 => {
                        let n_bits = bits.field(15)? as usize;
                        let mut sub_bits = bits.sub_stream(n_bits)?;
                        let mut sub_packets = Vec::new();
                        while let Some(packet) = Packet::from_bitstream(&mut sub_bits) {
                            sub_packets.push(packet);
                        }
                        Some(Self::Operator {
                            version,
                            sub_packets,
                            opcode,
                        })
                    }
                    1 => {
                        let n_packets = bits.field(11)?;
                        let sub_packets: Vec<_> = (0..n_packets)
                            .map(|_| Packet::from_bitstream(bits).unwrap())
                            .collect();
                        Some(Self::Operator {
                            version,
                            sub_packets,
                            opcode,
                        })
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal { version, .. } => *version,
            Packet::Operator {
                version,
                sub_packets,
                ..
            } => *version + sub_packets.iter().map(|p| p.version_sum()).sum::<u64>(),
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                sub_packets,
                opcode,
                ..
            } => match *opcode {
                0 => sub_packets.iter().map(|p| p.evaluate()).sum(),
                1 => sub_packets.iter().map(|p| p.evaluate()).product(),
                2 => sub_packets.iter().map(|p| p.evaluate()).min().unwrap(),
                3 => sub_packets.iter().map(|p| p.evaluate()).max().unwrap(),
                5 => (sub_packets[0].evaluate() > sub_packets[1].evaluate()) as u64,
                6 => (sub_packets[0].evaluate() < sub_packets[1].evaluate()) as u64,
                7 => (sub_packets[0].evaluate() == sub_packets[1].evaluate()) as u64,
                _ => unreachable!(),
            },
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut stream = BitStream::from(input.as_lines()[0]);
    let packet = Packet::from_bitstream(&mut stream);
    packet.unwrap().version_sum()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut stream = BitStream::from(input.as_lines()[0]);
    let packet = Packet::from_bitstream(&mut stream);
    packet.unwrap().evaluate()
}
