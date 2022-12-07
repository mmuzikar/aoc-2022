use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    fmt::Debug,
    ops::Deref,
    rc::Rc,
};

struct File {
    content: Vec<Rc<RefCell<File>>>,
    parent: Option<Rc<RefCell<File>>>,
    name: String,
    size: Option<u64>,
}

const MAX_MEMORY: u64 = 70000000;
const NEEDED_SPACE : u64 = 30000000;

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("File")
            .field("content", &self.content)
            .field("name", &self.name)
            .field("size", &self.size)
            .finish()
    }
}

impl File {
    fn get_file(&self, name: &str) -> Rc<RefCell<File>> {
        let file = self
            .content
            .iter()
            .find(|f| f.deref().deref().borrow().name == name)
            .expect("Couldn't find file");
        file.clone()
    }

    fn get_size(&self) -> u64 {
        if let Some(size) = self.size {
            size
        } else {
            self.content
                .iter()
                .map(|f| f.deref().borrow().get_size())
                .sum()
        }
    }

    fn get_bound_size(&self) -> u64 {
        let size = if self.get_size() > 100000 || self.size.is_some() {
            0
        } else {
            self.get_size()
        };
        size + self
            .content
            .iter()
            .map(|f| f.deref().borrow().get_bound_size())
            .sum::<u64>()
    }

    fn find_smallest_file(&self, unused_space: u64, min: &mut u64) -> u64 {
        if self.size.is_some() {
            return *min;
        }
        let size = self.get_size();
        if size + unused_space > NEEDED_SPACE {
            *min = if size < *min {
                size
            } else {
                *min
            }
        }
        for file in self.content.iter() {
            file.deref().borrow().find_smallest_file(unused_space, min);
        }
        *min
    }

}

fn parse_fs(s: &str) -> Rc<RefCell<File>> {
    let mut root: Rc<RefCell<File>> = Rc::new(RefCell::new(File {
        name: "/".to_string(),
        content: vec![],
        parent: None,
        size: None,
    }));
    let mut current = Rc::clone(&root);

    for ele in s.lines().map(|s| s.split(" ").collect::<Vec<_>>()).skip(1) {
        if ele.get(0).unwrap() == &"$" {
            if ele.get(1).unwrap() == &"cd" {
                match ele.get(2).unwrap() {
                    &".." => {
                        let parent = current.deref().borrow().parent.clone().unwrap();
                        current = parent;
                    }
                    &"/" => {
                        current = root.clone();
                    }
                    s => {
                        let child = current.deref().borrow().get_file(s);
                        current = child;
                    }
                }
            }
        } else {
            if ele.get(0).unwrap() == &"dir" {
                let name = ele.get(1).unwrap();

                let file = Rc::new(RefCell::new(File {
                    name: name.to_string(),
                    content: vec![],
                    parent: Some(current.clone()),
                    size: None,
                }));

                current
                    .borrow_mut()
                    .deref()
                    .deref()
                    .borrow_mut()
                    .content
                    .push(file.clone());
            } else {
                let size = ele
                    .get(0)
                    .unwrap()
                    .parse::<_>()
                    .expect("Failed to parse file size");
                let name = ele.get(1).unwrap().to_string();

                let current = current.borrow_mut();

                let mut file = Rc::new(RefCell::new(File {
                    content: vec![],
                    name,
                    parent: Some(current.clone()),
                    size: Some(size),
                }));

                current
                    .deref()
                    .deref()
                    .borrow_mut()
                    .content
                    .push(file.clone());
            }
        }
    }

    root
}

fn main() {
    let root = parse_fs(include_str!("input"));
    println!(
        "Total size: {:?}",
        root.deref().deref().borrow().get_bound_size()
    );

    let mut size = root.deref().deref().borrow().get_size();
    let unused_space = MAX_MEMORY - size;
    println!("Smallest size: {}", root.deref().deref().borrow().find_smallest_file(unused_space, &mut size));
}
