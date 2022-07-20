use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use bit_vec::BitVec;

fn load_from_file(file_path: &str) -> BitVec {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    let mut res = BitVec::new();
    for line_res in reader.lines() {
        if let Ok(line) = line_res {
            for ch in line.chars() {
                res.append(&mut get_bit_vec_from_char(ch));
            }
        }
    }

    res
}

fn get_bit_vec_from_char(ch: char) -> BitVec {
    let bv = 
        match ch {
            '0' => BitVec::from_bytes(&[0]),
            '1' => BitVec::from_bytes(&[1]),
            '2' => BitVec::from_bytes(&[2]),
            '3' => BitVec::from_bytes(&[3]),
            '4' => BitVec::from_bytes(&[4]),
            '5' => BitVec::from_bytes(&[5]),
            '6' => BitVec::from_bytes(&[6]),
            '7' => BitVec::from_bytes(&[7]),
            '8' => BitVec::from_bytes(&[8]),
            '9' => BitVec::from_bytes(&[9]),
            'A' => BitVec::from_bytes(&[10]),
            'B' => BitVec::from_bytes(&[11]),
            'C' => BitVec::from_bytes(&[12]),
            'D' => BitVec::from_bytes(&[13]),
            'E' => BitVec::from_bytes(&[14]),
            'F' => BitVec::from_bytes(&[15]),
            _ => panic!("impossible state"),
        };
    // keep only 4 last bits 
    let mut res = BitVec::new();
    res.push(bv[4]);
    res.push(bv[5]);
    res.push(bv[6]);
    res.push(bv[7]);
    res
}

#[derive(Debug)]
enum PacketData {
    Literal(u64),
    Operator(Vec<Packet>),
    None,
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    data: PacketData,
}

#[derive(Copy, Clone, Debug)]
enum ReadLiteralState {
    StartLiteral,
    StartGroup(u8),
    NonLastGroup(u8, u8),
    LastGroup(u8, u8),
}

#[derive(Debug)]
enum TraverseMode {
    ReadVersion(u8),
    ReadTypeId(u8),
    ReadLiteral(ReadLiteralState),
    
    ReadLengthTypeId,
    
    ReadBitLength(u8),
    ReadPacketCount(u8),
    
    ReadSubPacketsByBitCount(u16),
    ReadSubPacketsByPacketCount(u16),
}

// returns root packet
fn build_packet_tree(bit_vec: &BitVec, start_idx: usize, end_idx: usize) -> (usize, Packet) {
    let mut read_mode = TraverseMode::ReadVersion(0);
    let mut packet = Packet { version: 0, type_id: 0, data: PacketData::None };
    let mut mini_buf = 0u16; 
    
    let mut i = start_idx;
    while i < end_idx {
        let bit = bit_vec[i];
        
        // println!("{:?}: {:}", read_mode, bit as u8);
        match read_mode {
            TraverseMode::ReadVersion(tm_counter) => {
                packet.version <<= 1;
                packet.version |= bit as u8;
                if tm_counter == 2 {
                    read_mode = TraverseMode::ReadTypeId(0);
                } else {
                    read_mode = TraverseMode::ReadVersion(tm_counter + 1);
                }
            },
            TraverseMode::ReadTypeId(tm_counter) => {
                packet.type_id <<= 1;
                packet.type_id |= bit as u8;
                if tm_counter == 2 {
                    if packet.type_id == 4 {
                        read_mode = TraverseMode::ReadLiteral(ReadLiteralState::StartLiteral);
                    } else {
                        read_mode = TraverseMode::ReadLengthTypeId;
                    }
                } else {
                    read_mode = TraverseMode::ReadTypeId(tm_counter + 1);
                }
            },
            TraverseMode::ReadLengthTypeId => {
                if bit {
                    read_mode = TraverseMode::ReadPacketCount(0);
                } else {
                    read_mode = TraverseMode::ReadBitLength(0);
                }
            },
            TraverseMode::ReadLiteral(tm_counter) => {
                match tm_counter {
                    ReadLiteralState::StartLiteral => {
                        packet.data = PacketData::Literal(0);
                        if bit {
                            // 3 version bits + 3 len type id bits + 5 group bits = 11
                            read_mode = TraverseMode::ReadLiteral(ReadLiteralState::NonLastGroup(11, 0));
                        } else {
                            read_mode = TraverseMode::ReadLiteral(ReadLiteralState::LastGroup(11, 0));
                        }
                    },
                    ReadLiteralState::StartGroup(curr_bits) => {
                        if bit {
                            read_mode = TraverseMode::ReadLiteral(ReadLiteralState::NonLastGroup(curr_bits, 0));
                        } else {
                            read_mode = TraverseMode::ReadLiteral(ReadLiteralState::LastGroup(curr_bits, 0));
                        }
                    },
                    ReadLiteralState::NonLastGroup(curr_bits, rls_counter) => {
                        match packet.data {
                            PacketData::Literal(val) => {
                                packet.data = PacketData::Literal((val << 1) | bit as u64);
                            },
                            _ => {
                                panic!("Invalid PacketData for ReadLiteralState::NonLastGroup({:}): {:?}", rls_counter, packet.data);
                            }
                        }
                        
                        if rls_counter == 3 {
                            read_mode = TraverseMode::ReadLiteral(ReadLiteralState::StartGroup(curr_bits + 5));
                        } else {
                            read_mode = TraverseMode::ReadLiteral(ReadLiteralState::NonLastGroup(curr_bits, rls_counter + 1));
                        }
                    },
                    ReadLiteralState::LastGroup(curr_bits, rls_counter) => {
                        match packet.data {
                            PacketData::Literal(val) => {
                                packet.data = PacketData::Literal((val << 1) | bit as u64);
                            },
                            _ => {
                                panic!("Invalid PacketData for ReadLiteralState::LastGroup({:}): {:?}", rls_counter, packet.data);
                            }
                        }
                        
                        if rls_counter == 3 {
                            return (i + 1 - start_idx, packet); // END
                        } else {
                            read_mode = TraverseMode::ReadLiteral(ReadLiteralState::LastGroup(curr_bits, rls_counter + 1));
                        }
                    },
                }
            },
            TraverseMode::ReadBitLength(tm_counter) => {
                mini_buf <<= 1;
                mini_buf |= bit as u16;
                
                if tm_counter == 14 {
                    read_mode = TraverseMode::ReadSubPacketsByBitCount(mini_buf);
                    mini_buf = 0;
                } else {
                    read_mode = TraverseMode::ReadBitLength(tm_counter + 1);
                }
            },
            TraverseMode::ReadPacketCount(tm_counter) => {
                mini_buf <<= 1;
                mini_buf |= bit as u16;
                
                if tm_counter == 10 {
                    read_mode = TraverseMode::ReadSubPacketsByPacketCount(mini_buf);
                    mini_buf = 0;
                } else {
                    read_mode = TraverseMode::ReadPacketCount(tm_counter + 1);
                }
            },
            TraverseMode::ReadSubPacketsByPacketCount(packet_count) => {
                let mut packets = Vec::new();
                
                let mut new_start_idx = i;
                for _ in 0..packet_count {
                    let (bits_read, child_packet) = build_packet_tree(&bit_vec, new_start_idx, end_idx);
                    new_start_idx += bits_read;
                    packets.push(child_packet);
                }

                packet.data = PacketData::Operator(packets);
                
                // reading header for packet count costs 18 bits:
                // 3 (version) + 3 (type) + 1 (len id) + 11 (value)
                return (18 + new_start_idx - i, packet); // END
            },
            TraverseMode::ReadSubPacketsByBitCount(bit_count) => {
                let mut packets = Vec::new();
                
                let mut new_start_idx = i;
                let new_end_idx = new_start_idx + bit_count as usize;
                while new_start_idx < new_end_idx {
                    let (bits_read, child_packet) = build_packet_tree(&bit_vec, new_start_idx, new_end_idx);
                    new_start_idx += bits_read;
                    packets.push(child_packet);
                }

                packet.data = PacketData::Operator(packets);
                
                // reading header for bit count costs 22 bits:
                // 3 (version) + 3 (type) + 1 (len id) + 15 (value)
                return (22 + new_end_idx - i, packet); // END
            },
        }
        
        i += 1;
    }

    panic!("Unreachable part of code");
    // (end_idx - start_idx, packet)
}

fn get_version_sum(root_packet: &Packet) -> u64 {
    root_packet.version as u64 + match &root_packet.data {
        PacketData::Operator(vec) => vec.iter().fold(0u64, |acc, packet| acc + get_version_sum(packet)),
        _ => 0
    }
}

fn main() {
    let bit_vec = load_from_file("data.in");
    // println!("{:?}", bit_vec);
    let root_packet = build_packet_tree(&bit_vec, 0, bit_vec.len());
    // println!("{:#?}", root_packet.1);
    let version_sum = get_version_sum(&root_packet.1);
    println!("{:?}", version_sum);
}

