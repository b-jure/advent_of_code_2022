#![allow(dead_code)]

#[derive(Eq, PartialEq, Debug)]
pub struct File {
    identifier: String,
    size: usize,
}

impl File {
    fn new(identifier: &str, size: usize) -> Self {
        File { identifier: identifier.to_string(), size }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Folder {
    identifier: String,
    folders: Vec<Folder>,
    files: Vec<File>,
    outer: Option<*mut Folder>,
}

impl Folder {
    fn new(identifier: &str, outer: Option<*mut Folder>) -> Self {
        Folder { identifier: identifier.to_string(), folders: vec![], files: vec![], outer }
    }

    fn add_file(&mut self, file: File) -> &mut File {
        let idx = self.files.iter_mut().position(|f| &*f.identifier == &*file.identifier).unwrap_or_else(|| {
            self.files.push(file);
            self.files.len() - 1
        });

        &mut self.files[idx]
    }

    fn smallest(&self, needed: usize) -> &Folder {
        let mut min = vec![self];
        self.folders.iter().for_each(|folder| {
            if folder.size() >= needed { 
                min.push(folder.smallest(needed));
            }
        });
        min.sort_by_key(|folder| folder.size());
        min.first().unwrap()
    }

    fn add_folder(&mut self, id: &str) -> &mut Folder {
        let idx = self.folders.iter().position(|folder| &*folder.identifier == id).unwrap_or_else(|| {
            let folder = Folder::new(id, Some(self as *mut Folder));
            self.folders.push(folder);
            self.folders.len() - 1
        });
        &mut self.folders[idx]
    }

    fn rm_folder(&mut self, id: &str) {
        self.folders.retain(|folder| &*folder.identifier != id);
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        self.files.iter().for_each(|file| size += file.size);
        self.folders.iter().for_each(|folder| size += folder.size());
        size
    }

    fn is_within(&self, size: usize) -> bool {
        self.size() <= size
    }

    fn add_if_within<'folder>(&'folder self, size: usize, table: &mut Vec<&'folder Folder>) {
        self.folders.iter()
            .for_each(|folder| {
                if folder.is_within(size) { 
                    table.push(folder);
                }
                folder.add_if_within(size, table);
            });
    }
}

const MAX_SIZE: usize = 70_000_000;

pub struct Filesystem {
    root: Folder,
    current: *mut Folder,
}

impl Filesystem {
    pub fn new() -> Self {
        let root = Folder::new("/", None);
        let mut fs = Filesystem { root, current: std::ptr::null_mut() };
        fs.current = &mut fs.root;
        fs
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn smallest_dir(&self, size: usize) -> Option<&Folder> {
        let root_size = self.size();

        if root_size + size <= MAX_SIZE { 
            None 
        } else {
            let unused = MAX_SIZE - root_size;
            let needed = size - unused;
            Some(self.root.smallest(needed))
        }
    }

    pub fn read_line(&mut self, line: &str) {
        use Token::*;
        let token = Token::from(line);
        
        match token {
            Command(CommandKind::CD(id)) => self.cd(&*id),
            Command(CommandKind::LS) => (),
            Dir(id) => { self.add_folder(&*id); },
            File(file) => { self.add_file(file); },
        }
    }

    fn at_root(&mut self) -> bool {
        &*self.get_current().identifier == "/"
    }

    fn get_current(&mut self) -> &mut Folder {
        unsafe { &mut *self.current }
    }

    pub fn folders_within(&self, size: usize) -> Vec<&Folder> {
        let mut folders = Vec::new();   
        if self.root.is_within(size) {
            folders.push(&self.root);
        }
        self.root.add_if_within(size, &mut folders);
        folders
    }

    fn get_outer(&mut self) -> *mut Folder {
        if self.at_root() {
            self.current
        } else {
            self.get_current().outer.unwrap()
        }
    }

    fn get_folder(&mut self, id: &str) -> Option<&mut Folder> {
        self.get_current().folders.iter_mut().find(|folder| &*folder.identifier == id)
    }

    fn cd(&mut self, id: &str) {
        if id == ".." {
            self.current = self.get_current().outer.unwrap();
        } else {
            let new: *mut Folder = self.get_folder(id).expect("Directory doesn't exist.");
            self.current = new;
        }
    }

    pub fn add_folder(&mut self, id: &str) -> &mut Folder {
        self.get_current().add_folder(id)
    }

    pub fn remove_folder(&mut self, id: &str) {
        self.get_current().rm_folder(id);
    }

    fn add_file(&mut self, file: File) -> &mut File {
        self.get_current().add_file(file)
    }
}

#[derive(Debug)]
enum CommandKind {
    CD(String),
    LS,
}

#[derive(Debug)]
enum Token {
    Command(CommandKind),
    Dir(String),
    File(File),
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        use Token::*;

        let slices: Vec<&str> = value.trim().split_whitespace().collect();
        let size = if let Ok(size) = slices[0].parse::<usize>() { Some(size) } else { None };

        match value {
            val if val.starts_with("$ cd") => Command(CommandKind::CD(slices[2].to_string())),
            val if val.starts_with("$ ls") => Command(CommandKind::LS),
            val if val.starts_with("dir") => Dir(slices[1].to_string()),
            _ if size.is_some() => File(crate::File::new(slices[1], size.unwrap())),
            _ => unreachable!(),
        }
    }
}

pub fn parse_input(input: &str) -> Filesystem {
    let mut fs = Filesystem::new();
    input.lines().skip(1).for_each(|line| fs.read_line(line.trim()));
    fs
}

pub fn part_1(filesystem: &Filesystem, size: usize) -> usize {
    filesystem.folders_within(size).into_iter().fold(0, |acc, folder| acc + folder.size())
}