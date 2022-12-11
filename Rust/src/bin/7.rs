// https://adventofcode.com/2022/day/7

use std::cell::RefCell;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum Item {
    File(FileItem),
    Directory(Directory)
}

#[derive(Debug)]
struct FileItem {
    size: usize,
    name: String
}

#[derive(Debug)]
struct Directory {
    name: String,
    contents: Vec<Item>
}

impl Directory {
    fn find_by_name(&self, name: String) -> Option<&Item> {
        for c in &self.contents {
            let item_name = match c {
                Item::File(x) => x.name.clone(),
                Item::Directory(x) => x.name.clone()
            };
            if item_name == name {
                return Some(c)
            }
        }
        None
    }

    fn find_by_name_mut(&mut self, name: String) -> Option<&mut Item> {
        for c in &mut self.contents {
            let item_name = match c {
                Item::File(x) => x.name.clone(),
                Item::Directory(x) => x.name.clone()
            };
            if item_name == name {
                return Some(c)
            }
        }
        None
    }

    fn contents_size(&self) -> usize {
        let mut total = 0;
        for c in &self.contents {
            total = total + match c {
                Item::File(file) => file.size,
                Item::Directory(dir) => dir.contents_size()
            };
        }
        total
    }

    fn get_directories(&self) -> Vec<&Directory> {
        let mut directories = vec![];
        for c in &self.contents {
            if let Item::Directory(dir) = c {
                directories.push(dir);
                directories.append(&mut dir.get_directories())
            }
        }
        directories
    }
}

struct FileSystem {
    root: Directory,
    current_path: Vec<String>
}

impl FileSystem {
    fn new(root: Directory) -> Self {
        Self {
            root,
            current_path: vec![]
        }
    }

    fn current_dir(&self) -> &Directory {
        let mut current_dir = &self.root;
        for name in &self.current_path {
            let found_item = current_dir.find_by_name(name.clone()).unwrap();
            if let Item::Directory(dir) = found_item {
                current_dir = dir;
            } else {
                panic!("Invalid path")
            }
        }
        current_dir
    }

    fn current_dir_mut(&mut self) -> &mut Directory {
        let mut current_dir = &mut self.root;
        for name in &self.current_path {
            let found_item = current_dir.find_by_name_mut(name.clone()).unwrap();
            if let Item::Directory(dir) = found_item {
                current_dir = dir;
            } else {
                panic!("Invalid path")
            }
        }
        current_dir
    }

    fn change_dir(&mut self, command: String) {
        match &command[..] {
            ".." => {self.current_path.pop();},
            "/" => self.current_path = vec![],
            _ => {
                let current_dir = self.current_dir();
                let next_dir = current_dir.find_by_name(command).unwrap();

                if let Item::Directory(dir) = next_dir {
                    self.current_path.push(dir.name.clone())
                } else {
                    panic!("Tried to cd to file or invalid directory")
                }
            }
        }
    }

    fn process_new_items(&mut self, items: Vec<&str>) {
        for item in items {
            if item.starts_with("dir") {
                let current_dir = self.current_dir_mut();
                current_dir.contents.push( Item::Directory(Directory {name: item[4..].to_string(), contents: vec![]}) );
            } else {
                let (size, name) = item.split_once(' ').unwrap();
                let size_int: usize = size.parse().unwrap();
                self.current_dir_mut().contents.push( Item::File( FileItem {size: size_int.clone(), name: name.to_string()} ) )
            }
        }
    }
}

fn main() {
    let mut file = File::open("../data/7.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    let fs = RefCell::new(FileSystem::new(Directory {name: "root".to_string(), contents: vec![]}));

    let mut i = 0;
    while i < lines.len() {
        let current_command = &lines[i][2..];
        if current_command.starts_with("cd") {
            let mut mut_fs_ref = fs.borrow_mut();
            mut_fs_ref.change_dir((&current_command[3..]).to_string());
        }
        else if current_command.starts_with("ls") {
            let mut contained_items = vec![];

            loop {
                i += 1;
                let next_line = lines.get(i);
                let next_line_str = match next_line {
                    None => { break },
                    Some(&x) => x,
                };
                if next_line_str.starts_with("$") {
                    break;
                }
                contained_items.push(next_line_str);
            }

            let mut mut_fs_ref = fs.borrow_mut();
            mut_fs_ref.process_new_items(contained_items);
            continue;
        }
        i += 1
    }

    println!("Part 1");
    let mut total_size = 0;
    let fs_ref = fs.borrow();
    let all_dirs = fs_ref.root.get_directories();
    for &dir in &all_dirs {
        let s = dir.contents_size();
        if s < 100000 {
            total_size += s
        }
    }
    println!("{}", total_size);

    println!("Part 2");
    let space_to_free = 30000000 - (70000000 - fs_ref.root.contents_size());
    let mut valid_dirs = vec![];
    for &dir in &all_dirs {
        if dir.contents_size() > space_to_free {
            valid_dirs.push(dir)
        }
    }

    valid_dirs.sort_by(|&a, &b| a.contents_size().cmp(&b.contents_size()));
    println!("{}", valid_dirs.get(0).unwrap().contents_size());
}
