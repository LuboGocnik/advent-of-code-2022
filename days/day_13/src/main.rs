use crate::PacketData::{List, Num};
use std::cmp::Ordering;

const CRLF: &str = "\r\n";
const LF: &str = "\n";

#[derive(Debug, Clone, Eq)]
enum PacketData {
    Num(usize),
    List(Vec<PacketData>),
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Num(num1), Num(num2)) => num1.cmp(num2),
            (Num(num), List(list)) => List(vec![Num(*num)]).cmp(&List(list.clone())),
            (List(list), Num(num)) => List(list.clone()).cmp(&List(vec![Num(*num)])),
            (List(list1), List(list2)) => {
                let mut iter1 = list1.iter();
                let mut iter2 = list2.iter();

                let mut answer = Ordering::Equal;

                loop {
                    let v1 = iter1.next();
                    let v2 = iter2.next();

                    match (v1, v2) {
                        (None, None) => break,
                        (Some(_), None) => {
                            answer = Ordering::Greater;
                            break;
                        }
                        (None, Some(_)) => {
                            answer = Ordering::Less;
                            break;
                        }
                        (Some(v1), Some(v2)) => match v1.cmp(&v2) {
                            Ordering::Equal => answer = Ordering::Equal,
                            order => {
                                answer = order;
                                break;
                            }
                        },
                    }
                }

                answer
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn main() {
    let input = include_str!("input.txt");
    let new_line = if input.contains(CRLF) { CRLF } else { LF };
    let empty_line = format!("{}{}", new_line, new_line);

    let packet_pairs: Vec<(PacketData, PacketData)> = input
        .split(&empty_line)
        .map(process_raw_packet_pairs)
        .collect();

    let mut packets: Vec<PacketData> = input
        .split(&empty_line)
        .map(process_raw_packet_pairs)
        .flat_map(|c| vec![c.0, c.1])
        .collect();

    let delimiters = [
        List(vec![List(vec![Num(2)])]),
        List(vec![List(vec![Num(6)])]),
    ];
    packets.push(delimiters[0].clone());
    packets.push(delimiters[1].clone());

    packets.sort();

    let first: usize = packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (c.0.cmp(&c.1) == Ordering::Less).then(|| i + 1))
        .sum();

    let second: usize = packets
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (c.eq(&delimiters[0]) || (c.eq(&delimiters[1]))).then(|| i + 1))
        .reduce(|v1, v2| v1 * v2)
        .unwrap();

    println!("First: {}", first);
    println!("Second: {}", second);
}

fn process_raw_packet_pairs(line: &str) -> (PacketData, PacketData) {
    let new_line = if line.contains(CRLF) { CRLF } else { LF };
    let packets = line.split_once(new_line).unwrap();

    (process_packet(packets.0), process_packet(packets.1))
}

fn process_packet(s: &str) -> PacketData {
    if &s[0..1] == "[" {
        let mut stack: i32 = 0;
        List(
            s[1..s.len() - 1]
                .split(|c| {
                    if c == '[' {
                        stack += 1
                    } else if c == ']' {
                        stack -= 1
                    }
                    c == ',' && stack == 0
                })
                .filter_map(|s| (!s.is_empty()).then(|| process_packet(s)))
                .collect(),
        )
    } else {
        Num(s.parse().unwrap())
    }
}
