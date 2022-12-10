use std::{fmt, str::FromStr};

use indextree::{Arena, Node, NodeEdge, NodeId};

pub(crate) struct Folder {
    name: String,
    id: Option<NodeId>,
}

pub(crate) struct File {
    size: u32,
    name: String,
}

pub(crate) enum Directory {
    Folder(Folder),
    File(File),
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Folder(folder) => write!(f, "{} (folder)", folder.name),
            Self::File(file) => write!(f, "{} (file, size={})", file.name, file.size),
        }
    }
}

impl Directory {
    fn size(&self, file_system: &FileSystem) -> u32 {
        match self {
            Directory::Folder(Folder { id, .. }) => id
                .unwrap()
                .children(&file_system.arena)
                .filter_map(|child| file_system.arena.get(child).map(Node::get))
                .map(|directory| directory.size(file_system))
                .sum(),
            Directory::File(File { size, .. }) => *size,
        }
    }
}

pub(crate) struct FileSystem {
    pub(crate) arena: Arena<Directory>,
    pub(crate) root: NodeId,
}

fn new_folder_node(name: String, arena: &mut Arena<Directory>) -> NodeId {
    let node_id = arena.new_node(Directory::Folder(Folder { name, id: None }));
    if let Directory::Folder(folder) = arena.get_mut(node_id).unwrap().get_mut() {
        folder.id = Some(node_id);
    }
    node_id
}

impl FileSystem {
    fn parse_lines(lines: Vec<Line>) -> Self {
        let mut arena = Arena::new();

        let root = new_folder_node("/".to_owned(), &mut arena);
        let mut working_directory = root;

        for line in lines {
            match line {
                Line::Command(Command::Cd(Cd::Dir(dir))) => {
                    let folder = new_folder_node(dir.clone(), &mut arena);
                    working_directory.append(folder, &mut arena);
                    working_directory = folder;
                }
                Line::Command(Command::Cd(Cd::Parent)) => {
                    working_directory = working_directory.ancestors(&arena).nth(1).unwrap_or(root);
                }
                Line::Command(Command::Cd(Cd::Root)) => working_directory = root,
                Line::Output(Output::File { name, size }) => {
                    let file = arena.new_node(Directory::File(File {
                        size,
                        name: name.clone(),
                    }));
                    working_directory.append(file, &mut arena);
                }
                _ => {}
            }
        }

        FileSystem { arena, root }
    }

    pub(crate) fn sizes(&self) -> Vec<u32> {
        self.root
            .traverse(&self.arena)
            .filter_map(|node_edge| match node_edge {
                NodeEdge::Start(node_id) => Some(node_id),
                NodeEdge::End(_) => None,
            })
            .filter_map(|child| self.arena.get(child).map(Node::get))
            .filter(|directory| matches!(directory, Directory::Folder(..)))
            .map(|directory| directory.size(self))
            .collect()
    }
}

enum Line {
    Command(Command),
    Output(Output),
}

enum Command {
    Cd(Cd),
    Ls,
}

enum Cd {
    Dir(String),
    Parent,
    Root,
}

enum Output {
    Folder { _name: String },
    File { name: String, size: u32 },
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut word = s.split_whitespace();
        match (word.next(), word.next(), word.next()) {
            (Some("$"), Some("ls"), None) => Ok(Line::Command(Command::Ls)),
            (Some("$"), Some("cd"), Some(dir)) => Ok(Line::Command(Command::Cd(match dir {
                ".." => Cd::Parent,
                "/" => Cd::Root,
                dir => Cd::Dir(dir.to_owned()),
            }))),
            (Some(first), Some(second), None) => {
                if let Ok(size) = first.parse() {
                    Ok(Line::Output(Output::File {
                        name: second.to_owned(),
                        size,
                    }))
                } else {
                    Ok(Line::Output(Output::Folder {
                        _name: second.to_owned(),
                    }))
                }
            }
            _ => Err("Cannot parse line!"),
        }
    }
}

pub(crate) fn parse(input: &str) -> FileSystem {
    FileSystem::parse_lines(
        input
            .lines()
            .map(Line::from_str)
            .filter_map(Result::ok)
            .collect(),
    )
}

pub fn solve_part1(input: &str) -> u32 {
    parse(input)
        .sizes()
        .into_iter()
        .filter_map(|size| (size < 100_000).then_some(size))
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let fs = parse(input);

    let current_size = fs.arena.get(fs.root).unwrap().get().size(&fs);

    fs.sizes()
        .into_iter()
        .filter(|size| 70_000_000 - (current_size - size) > 30_000_000)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day07::{parse, solve_part1, solve_part2};

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1() {
        let fs = parse(INPUT);
        println!("{:#?}", fs.root.debug_pretty_print(&fs.arena));
        assert_eq!(solve_part1(INPUT), 95437);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 24_933_642);
    }
}
