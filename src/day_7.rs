use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{newline, space1, u64},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

pub fn solution(input: &str) -> String {
    let command_and_result = parse(input);
    format!(
        "{}, {}",
        part_one(&command_and_result),
        part_two(&command_and_result)
    )
}

#[derive(Debug)]
enum CdPath<'a> {
    Absolute(&'a str),
    Relative(&'a str),
    Parent,
}

#[derive(Debug)]
enum LsEntry<'a> {
    File(u64, &'a str),
    Dir(&'a str),
}

#[derive(Debug)]
enum CommandAndResult<'a> {
    Cd(CdPath<'a>),
    Ls(Vec<LsEntry<'a>>),
}

use CommandAndResult::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FSPath<'a>(Vec<&'a str>);

impl<'a> FSPath<'a> {
    fn from_absolute(absolute: &'a str) -> Self {
        assert!(absolute.starts_with('/'));
        let segments = absolute[1..].split('/').collect();
        Self(segments)
    }

    fn push(&mut self, relative: &'a str) {
        self.0.push(relative)
    }

    fn pop(&mut self) {
        self.0.pop();
    }

    fn is_ancestor(&self, other: &Self) -> bool {
        other.0.starts_with(&self.0)
    }
}

fn dir_sizes<'a>(command_and_result: &'a [CommandAndResult]) -> HashMap<FSPath<'a>, u64> {
    let mut file_system: HashMap<FSPath, Vec<(&str, u64)>> = HashMap::new();
    let mut pwd = FSPath::from_absolute("/");

    for comm in command_and_result {
        match comm {
            Cd(CdPath::Absolute(path)) => pwd = FSPath::from_absolute(path),
            Cd(CdPath::Relative(path)) => pwd.push(path),
            Cd(CdPath::Parent) => pwd.pop(),
            Ls(entries) => {
                let files = entries
                    .iter()
                    .filter_map(|entry| {
                        if let LsEntry::File(size, name) = entry {
                            Some((*name, *size))
                        } else {
                            None
                        }
                    })
                    .collect();

                file_system.insert(pwd.clone(), files);
            }
        }
    }

    file_system
        .keys()
        .map(|p| {
            let total_size = file_system
                .iter()
                .filter_map(|(c, files)| {
                    if p.is_ancestor(c) {
                        let sum = files.iter().map(|(_, size)| size).sum::<u64>();
                        Some(sum)
                    } else {
                        None
                    }
                })
                .sum::<u64>();

            (p.clone(), total_size)
        })
        .collect()
}

fn part_one(command_and_result: &[CommandAndResult]) -> u64 {
    let dir_sizes = dir_sizes(command_and_result);
    dir_sizes
        .iter()
        .map(|(_, &size)| if size <= 100000 { size } else { 0 })
        .sum()
}

fn part_two(command_and_result: &[CommandAndResult]) -> u64 {
    let dir_sizes = dir_sizes(command_and_result);
    let total_size = dir_sizes[&FSPath::from_absolute("/")];
    let required = 30_000_000 - (70_000_000 - total_size);

    dir_sizes
        .into_values()
        .filter(|&size| size >= required)
        .min()
        .expect("valid input")
}

fn parse(input: &str) -> Vec<CommandAndResult> {
    let (remain, command_and_result) = p_input(input).expect("valid input");
    if !remain.is_empty() {
        panic!("parse incomplete");
    }
    command_and_result
}

fn p_input(input: &str) -> IResult<&str, Vec<CommandAndResult>> {
    separated_list1(newline, p_command_and_result)(input)
}

fn p_command_and_result(input: &str) -> IResult<&str, CommandAndResult> {
    alt((
        preceded(tag("$ cd "), p_cd_path).map(CommandAndResult::Cd),
        preceded(tag("$ ls\n"), p_ls_result).map(CommandAndResult::Ls),
    ))(input)
}

fn p_cd_path(input: &str) -> IResult<&str, CdPath> {
    let (input, path) = take_while(|c: char| c != '\n')(input)?;
    let cd_path = match path {
        ".." => CdPath::Parent,
        _ if path.starts_with('/') => CdPath::Absolute(path),
        _ => CdPath::Relative(path),
    };

    Ok((input, cd_path))
}

fn p_ls_result(input: &str) -> IResult<&str, Vec<LsEntry>> {
    separated_list0(newline, p_ls_entry)(input)
}

fn p_ls_entry(input: &str) -> IResult<&str, LsEntry> {
    alt((
        preceded(tag("dir "), take_while(|c: char| c != '\n')).map(LsEntry::Dir),
        separated_pair(u64, space1, take_while(|c: char| c != '\n'))
            .map(|(size, name)| LsEntry::File(size, name)),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_tests() {
        assert!(p_ls_entry("192236 vmtnnfv.mdq").is_ok());
        assert!(p_ls_entry("dir vmvpf").is_ok());
        let (remain, _) = p_ls_result(
            "dir bqm
dir ctztn
dir dbclg
dir fhndmnt
dir gczqbh
276177 hvbf.lvm
dir lnsgbqp
dir pblb
dir pwfs
209719 rtv.cjj
192236 vmtnnfv.mdq
dir vmvpf
dir wjgh
dir wjqsqn",
        )
        .unwrap();

        assert_eq!(remain.len(), 0);
    }
}
