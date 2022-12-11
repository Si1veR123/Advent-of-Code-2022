# https://adventofcode.com/2022/day/7

import typing

class Item:
    # any item in filesystem (file or directory)
    def __init__(self, name):
        self.name = name

class File(Item):
    def __init__(self, name, size):
        super().__init__(name)
        self.size = size

class Directory(Item):
    def __init__(self, name):
        super().__init__(name)
        self.contents: typing.List[Item] = []
    
    def add_contents(self, contents: Item):
        self.contents.append(contents)
    
    def find_by_name(self, search_name):
        for c in self.contents:
            if c.name == search_name:
                return c

    def contents_size(self):
        # recursively find folder size
        total = 0
        for c in self.contents:
            if isinstance(c, File):
                total += c.size
            elif isinstance(c, Directory):
                total += c.contents_size()
        return total
    
    def get_directories(self):
        # recursively get all directories
        directories = []
        for c in self.contents:
            if isinstance(c, Directory):
                directories.append(c)
                directories += c.get_directories()
        return directories

class FileSystem:
    def __init__(self):
        self.root = Directory("root")
        self.current_path: typing.List[Directory] = [self.root]

    def change_dir(self, command):
        if command == "..":
            self.current_path.pop()
        elif command == "/":
            self.current_path = [self.root]
        else:
            next_dir = self.current_dir.find_by_name(command)
            if not isinstance(next_dir, Directory):
                raise TypeError("Tried to cd to file or invalid directory")
            self.current_path.append(next_dir)

    @property
    def current_dir(self):
        return self.current_path[-1]
    
    def process_new_items(self, items: typing.List[str]):
        for item in items:
            if item.startswith("dir"):
                self.current_dir.add_contents(Directory(item[4:]))
            else:
                size, name = item.split()
                size = int(size)
                self.current_dir.add_contents(File(name, size))

with open("../data/7.txt") as file:
    lines = file.read().splitlines()

fs = FileSystem()

i = 0
while i < len(lines):
    current_command = lines[i][2:]
    if current_command.startswith("cd"):
        fs.change_dir(current_command[3:])
    elif current_command.startswith("ls"):
        contained_items = []

        while 1:
            i += 1
            try:
                next_line = lines[i]
            except IndexError:
                break
            if next_line.startswith("$"):
                break
            contained_items.append(next_line)

        fs.process_new_items(contained_items)
        continue  # continue as next line is a command, dont increment
    i += 1

print("Part 1")
total_size = 0
all_dirs = fs.root.get_directories()
for dir in all_dirs:
    s = dir.contents_size()
    if s < 100000:
        total_size += s
print(total_size)

print("Part 2")
space_to_free = 30000000 - (70000000 - fs.root.contents_size())
valid_dirs = []
for dir in all_dirs:
    if dir.contents_size() > space_to_free:
        valid_dirs.append(dir)

print(sorted(valid_dirs, key=lambda x: x.contents_size())[0].contents_size())
