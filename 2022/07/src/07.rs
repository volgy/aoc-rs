use std::collections::HashMap;

aoc::parts!(1, 2);

#[derive(Debug)]
enum Node {
    Directory(Directory),
    File(File),
}

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug, Default)]
struct Directory {
    children: HashMap<String, Node>,
}

impl Directory {
    fn process_transcript(&mut self, lines: &mut aoc::Lines) {
        while let Some(line) = lines.next() {
            if !line.starts_with("$") {
                if line.starts_with("dir") {
                    self.children
                        .insert(line[4..].to_owned(), Node::Directory(Directory::default()));
                } else {
                    let (size, name) = line.split_once(" ").unwrap();
                    self.children.insert(
                        name.to_owned(),
                        Node::File(File {
                            size: size.parse().unwrap(),
                        }),
                    );
                }
            } else if line == "$ cd .." {
                return;
            } else if line.starts_with("$ cd") {
                match self.children.get_mut(&line[5..]) {
                    Some(Node::Directory(subdir)) => subdir.process_transcript(lines),
                    _ => panic!("invalid sub dirertory"),
                }
            } else if line != "$ ls" {
                panic!("invalid line")
            }
        }
    }

    fn calculate_total_size(&self) -> usize {
        let mut total_size = 0;

        for node in self.children.values() {
            match node {
                Node::File(file) => {
                    total_size += file.size;
                }
                Node::Directory(sub_dir) => {
                    total_size += Self::calculate_total_size(sub_dir);
                }
            }
        }

        total_size
    }

    fn directory_sizes(&self) -> DirectorySizeIterator {
        DirectorySizeIterator::new(self)
    }
}

struct DirectorySizeIterator<'a> {
    stack: Vec<&'a Directory>,
}

impl<'a> DirectorySizeIterator<'a> {
    fn new(root: &'a Directory) -> Self {
        Self { stack: vec![root] }
    }
}

// Inefficient (calculates aggregate directory sizes multiple times)
impl<'a> Iterator for DirectorySizeIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_dir) = self.stack.pop() {
            // Push all subdirectories onto the stack for future traversal.
            for node in current_dir.children.values() {
                if let Node::Directory(sub_dir) = node {
                    self.stack.push(sub_dir);
                }
            }

            // Return the total size of the current directory (including subdirectories).
            Some(current_dir.calculate_total_size())
        } else {
            None
        }
    }
}

fn build_tree(input: aoc::Input) -> Directory {
    let mut lines = input.lines().into_iter();
    // kind of hacky: we assume this happens exactly once at the beginning
    assert!(lines.next().unwrap() == "$ cd /");
    let mut root = Directory::default();
    root.process_transcript(&mut lines);
    root
    // eprintln!("root = {:#?}", root);
}
fn part_1(input: aoc::Input) -> impl ToString {
    build_tree(input)
        .directory_sizes()
        .filter(|&size| size <= 100_000)
        .sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let root = build_tree(input);
    let unused = 70_000_000 - root.calculate_total_size();
    let needed = 30_000_000 - unused;

    let mut sizes: Vec<_> = root.directory_sizes().collect();
    sizes.sort();
    for size in sizes {
        if size >= needed {
            return size;
        }
    }

    panic!("unable to find a single directory to be deleted")
}
