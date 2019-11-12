#![allow(dead_code, unused_variables, unused_imports)]

extern crate nom;
extern crate structopt;
extern crate chrono;

use std::path::PathBuf;
use structopt::StructOpt;
use std::error::Error;
use std::fs::OpenOptions;
use nom::IResult;
use nom::sequence::tuple;
use nom::number::complete::{le_u32, le_u64, le_f64};
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use std::io::{Read, Write};
use nom::combinator::{verify, map_res};
use nom::multi::count;
use std::convert::TryInto;
use chrono::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "fpffparse", about = "Commandline app to parse FPFF files.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read given input fpff file
    let opt = Opt::from_args();
    let input_path: PathBuf = opt.input;
    let mut file = OpenOptions::new().read(true).open(input_path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Parse fpff file
    let (bytes, (header, sections)) = parse_fpff(&bytes).unwrap();

    // Print Header info
    println!("This file was authored on {} by {}.", header.timestamp, header.author);

    // Print Section info
    for (i, section) in sections.iter().enumerate() {
        let i = i + 1;
        print!("Section {} is ", i);
        match section {
            Section::Ascii(s) => println!("ASCII of {:?}.", s),
            Section::Utf8(s) => println!("UTF8 of {:?}.", s),
            Section::Words(ws) => println!("Words of {:?}.", ws.as_slice()),
            Section::Dwords(ws) => println!("Dwords of {:?}.", ws.as_slice()),
            Section::Doubles(ws) => println!("Doubles of {:?}.", ws.as_slice()),
            Section::Coord {x, y} => println!("A Coord of x: {}, y: {}.", x, y),
            Section::Reference(n) => println!("a Reference to Section {}", n + 1),
            Section::Png(bs) => {
                println!("a PNG. Saving as {}.png.", i);
                output_file_section(*bs, FileType::Png, i)?;
            },
            Section::Gif87(bs) => {
                println!("a Gif87. Saving as {}.gif.", i);
                output_file_section(*bs, FileType::Gif87, i)?;
            },
            Section::Gif89(bs) => {
                println!("a Gif89. Saving as {}.gif.", i);
                output_file_section(*bs, FileType::Gif89, i)?;
            },
        }
    }

    Ok(())
}

fn parse_fpff(bytes: &[u8]) -> IResult<&[u8], (Header, Vec<Section>)> {
    let (mut bytes, header) = parse_header(&bytes).unwrap();

    let mut sections = Vec::new();
    for i in 0..header.section_count {
        let (bs, section) = parse_section(bytes, header.section_count)?;
        bytes = bs;
        sections.push(section);
    }

    Ok((bytes, (header, sections)))
}

#[derive(Debug)]
struct Header<'a> {
    timestamp: DateTime<Utc>,
    author: &'a str,
    section_count: u32
}

const MAGIC: u32 = 0x8BADF00D;
const MAGIC_BYTES: &[u8] = &[0x0D, 0xF0, 0xAD, 0x8B]; // LE order
const VERSION: u32 = 1;
const VERSION_BYTES: &[u8] = &[0x01, 0, 0, 0];

fn parse_header(bytes: &[u8]) -> IResult<&[u8], Header> {
    let (bytes,(_magic, _version, timestamp, author, section_count)) = tuple((
        tag(MAGIC_BYTES), tag(VERSION_BYTES),
        le_u32, // Date (should be that every number is unix time valid)
        verify(take(8u32), header_author_verify), // Author
        le_u32 // Section Count
    ))(bytes)?;

    // Can unwrap because we've already guaranteed it's ascii
    let author = std::str::from_utf8(author).unwrap()
        // Remove the nul bytes at the end.
        .split_terminator('\0').next().unwrap();

    let timestamp = Utc.timestamp(timestamp.into(), 0);

    Ok((bytes, Header {timestamp, author, section_count}))
}

fn header_author_verify(bytes: &[u8]) -> bool {
    let mut reached_zero = false;
    let mut author_correct = true;
    for &b in bytes {
        if !reached_zero && b == 0 {
            reached_zero = true;
        } else if reached_zero && b != 0 {
            author_correct = false;
        }
    }

    author_correct && bytes.iter().all(|b| b.is_ascii())
}

#[derive(Debug)]
enum Section<'a> {
    Ascii(&'a str), // 0x1
    Utf8(&'a str), // 0x2
    Words(Vec<u32>), // 0x3
    Dwords(Vec<u64>), // 0x4
    Doubles(Vec<f64>), // 0x5
    Coord {x: f64, y: f64}, // 0x6
    Reference(u32), // 0x7
    Png(&'a [u8]), // 0x8
    Gif87(&'a [u8]), // 0x9
    Gif89(&'a [u8]) // 0xA
}

fn parse_section(bytes: &[u8], section_count: u32) -> IResult<&[u8], Section> {
    let (bytes, stype) = verify( le_u32,
                                  |&stype| stype >= 0x1 && stype <= 0xA)
        (bytes)?;
    //dbg!(stype);
    let (bytes, slen) = verify(le_u32, |&slen| match stype {
        0x3 => slen % 4 == 0,
        0x4 | 0x5 => slen % 8 == 0,
        0x6 => slen == 16, // Coord must have 2 doubles
        0x7 => slen == 4,  // Reference must have 1 word
        _   => true,
    })(bytes)?;
    //dbg!(slen);

    let (bytes, ans) = match stype {
        0x1 => {
            let (bytes, bs) = verify(take(slen),
                   |bs: &[u8]| bs.iter().all(|b| b.is_ascii())
            )(bytes)?;
            // Can unwrap because we've already guaranteed it's ascii
            let asciistr = std::str::from_utf8(bs).unwrap()
                // Remove the nul bytes at the end.
                .split_terminator('\0').next().unwrap();

            (bytes, Section::Ascii(asciistr))
        },
        0x2 => {
            let (bytes, utfstr) = map_res(take(slen),
                                      |bs: &[u8]| std::str::from_utf8(bs)
            )(bytes)?;

            (bytes, Section::Utf8(utfstr))
        },
        0x3 => {
            let (bytes, ws) = count(le_u32, (slen/4).try_into().unwrap())(bytes)?;

            (bytes, Section::Words(ws))
        },
        0x4 => {
            let (bytes, ws) = count(le_u64, (slen/8).try_into().unwrap())(bytes)?;

            (bytes, Section::Dwords(ws))
        },
        0x5 => {
            let (bytes, fs) = count(le_f64, (slen/8).try_into().unwrap())(bytes)?;

            (bytes, Section::Doubles(fs))
        },
        0x6 => {
            let (bytes, (x, y)) = tuple((le_f64, le_f64))(bytes)?;

            (bytes, Section::Coord {x, y})
        },
        0x7 => {
            let (bytes, n) = verify(le_u32,
                                    |&n| n < section_count
            )(bytes)?;

            (bytes, Section::Reference(n))
        },
        0x8 => {
            let (bytes, bs) = take(slen)(bytes)?;

            (bytes, Section::Png(bs))
        },
        0x9 => {
            let (bytes, bs) = take(slen)(bytes)?;

            (bytes, Section::Gif87(bs))
        },
        0xA => {
            let (bytes, bs) = take(slen)(bytes)?;

            (bytes, Section::Gif89(bs))
        },
        _ => unreachable!()
    };

    Ok((bytes, ans))
}

enum FileType {
    Png, Gif87, Gif89
}
fn output_file_section(bs: &[u8], ft: FileType, n: usize) -> Result<(), Box<dyn Error>> {
    let mut options = OpenOptions::new();
    options.create(true).write(true).truncate(true);
    let mut file;

    match ft {
        FileType::Png => {
            file = options.open(format!("{}.png", n))?;
            // Write out PNG file signature
            file.write_all(&[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a])?;
        },
        FileType::Gif87 => {
            file = options.open(format!("{}.png", n)).unwrap();
            file.write_all("GIF87a".as_bytes())?;
        },
        FileType::Gif89 => {
            file = options.open(format!("{}.png", n)).unwrap();
            file.write_all("GIF89a".as_bytes())?;
        }
    }

    file.write_all(bs)?;

    Ok(())
}