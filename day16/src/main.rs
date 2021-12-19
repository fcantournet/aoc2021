
use nom::{
    bits::complete::tag,
    bits::complete::take,
    branch::alt,
    combinator::{flat_map, map},
    multi::{length_count, many0},
    sequence::{pair, preceded},
    IResult, InputLength,
};
use hex;

fn main() {
    let input = include_str!("input.txt");
    println!("Result 1 : {:?}", solve_1(input));
    println!("Result 2 : {:?}", solve_2(input));
}

enum PacketType{
    Operator(Vec<Packet>),
    Literal(usize)
}

struct Packet {
    kind: PacketType,
    version: usize,
    type_id: u8,
}

type Input<'a> = (&'a [u8], usize);

fn parse(input: &str) -> Packet {
    let buf = hex::decode(input).unwrap();
    packet((&buf, 0usize)).unwrap().1
}


fn leading_bits<'a>(input: Input<'a> ) -> IResult<Input<'a>, usize> {
    preceded(tag(1, 1usize), take(4usize))(input)
}
fn trailing_bits<'a>(input: Input<'a> ) -> IResult<Input<'a>, usize> {
    preceded(tag(0, 1usize), take(4usize))(input)
}

fn literal<'a>(input: Input<'a>) -> IResult<Input<'a>, PacketType> {
    map(
        pair(many0(leading_bits), trailing_bits),
        |(leading, trailing)| {
            PacketType::Literal(
            leading.iter().fold(0usize, |acc, bits| (acc << 4) + bits) << 4 + trailing
            )}
    )(input)
}


fn operator<'a>(input: Input<'a>) -> IResult<Input<'a>, PacketType> {
    map(
        alt((
            preceded(
                tag(0, 1usize),
                flat_map(take(15usize), |n: usize| {
                    move |mut input: (&'a [u8], usize)| {
                        let mut packets = Vec::new();
                        let input_len = input.input_len();

                        while input_len - input.input_len() < n {
                            let (remaining_input, next_packet) = packet(input)?;
                            packets.push(next_packet);
                            input = remaining_input;
                        }

                        // TODO: return an error here. nom error handling is a pain
                        assert_eq!(input_len - input.input_len(), n);

                        Ok((input, packets))
                    }
                }),
            ),
            preceded(
                tag(1, 1usize),
                length_count(map(take(11usize), |n: usize| n), packet),
            ),
        )),
        |operands| PacketType::Operator(operands),
    )(input)
}


fn packet(input: Input) -> IResult<Input, Packet> {

    map(
        pair(
            take(3usize),
            flat_map(take(3usize), |type_id| {
                map(
                    match type_id {
                        4 => literal,
                        _ => operator,
                    },
                    move |kind| (type_id, kind),
                )
            }),
        ),
        |(version, (type_id, kind))| Packet {
            version,
            type_id,
            kind,
        },
    )(input)
}




fn solve_2(input: &str) -> usize {
    0
}

fn sum_versions(packet: &Packet) -> usize {
    packet.version + match &packet.kind {
        PacketType::Literal(_) => 0,
        PacketType::Operator(vec) => {
            vec.iter().map(|p| sum_versions(p)).sum()
        }
    }
}

fn solve_1(input: &str) -> usize {
    let packet = parse(input);
    sum_versions(&packet)
}


#[test]
fn test_sample1() {
    let input = "8A004A801A8002F478";
    assert_eq!(solve_1(&input), 16);

    let input = "620080001611562C8802118E34";
    assert_eq!(solve_1(&input), 12);

    let input = "C0015000016115A2E0802F182340";
    assert_eq!(solve_1(&input), 23);

    let input = "A0016C880162017C3686B18A3D4780";
    assert_eq!(solve_1(&input), 31);
}


#[test]
fn test_sample2() {
    // let input = include_str!("sample.txt");
    // assert_eq!(solve_2(&input), 12)
}