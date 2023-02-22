use std::fs;
use regex::Regex;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Default)]
enum NodeType {
    #[default]
    Directory,
    File,
}
type NodeLink = Option<Rc<RefCell<Node>>>;


#[derive(Debug, Clone, Default)]
struct Node {
    ntype: NodeType,
    path: String,
    size: usize,
    parent: NodeLink,
    children: Vec<NodeLink>,
}

#[derive(Debug, Clone, Default)]
struct Fs {
    size: usize,
    root: NodeLink,
    curr: NodeLink,
}

impl Fs {

    fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    fn set_root(&mut self, root: NodeLink) {
        self.root = root;
    }
    
    fn set_curr(&mut self, curr: NodeLink) {
        self.curr = curr;
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_root(&self) -> NodeLink {
        self.root.clone()
    }

    fn get_current(&self) -> NodeLink {
        self.curr.clone()
    }

    fn get_parent(&self) -> NodeLink {
        let parent_path = self.curr.as_deref().unwrap().borrow().parent.as_deref().unwrap().borrow().path.clone();
        if let Some(node) = find_child(self.root.clone(), parent_path) {
            Some(node)
        } else {
            self.root.clone()
        }
    }

    fn add_child(&mut self, nchild: NodeLink) {
        self.curr.as_ref().unwrap().borrow_mut().add_child(nchild);
    }

    fn print_nodes(&self) {
        print_nodes(self.root.clone(), 2);
    }
}

impl Node {
    fn new(ntype: NodeType, path: String, size: usize) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Node { ntype, path, size, parent: None, children: vec![] }
            )
        )
    } 

    fn from_node(&self) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Node { 
                    ntype: self.ntype.clone(),
                    parent: self.parent.clone(),
                    children: self.children.clone(),
                    path: self.path.clone(),
                    size: self.size.clone()
                }
            )
        )
    }

    fn add_child(&mut self, nchild: NodeLink) {
        let mut path = String::default();
        if self.path == "/" {
            path = self.path.clone() + nchild.as_deref().unwrap().borrow().path.as_str().clone(); 
        } else {
            path = self.path.clone() + "/" + nchild.as_deref().unwrap().borrow().path.as_str().clone(); 
        }
        nchild.as_ref().unwrap().borrow_mut().path = path;
        nchild.as_ref().unwrap().borrow_mut().parent = Some(self.from_node());

        self.children.push(nchild);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

fn print_nodes(node: NodeLink, level: usize) {
    println!("");
    let space = String::from(" ").repeat(level - 1);
    print!("{}", space);
    match node.as_deref().unwrap().borrow().ntype {
        NodeType::Directory => {
            println!("Dir: size={:09}  path={}", node.as_deref().unwrap().borrow().size, node.as_deref().unwrap().borrow().path);
        },
        NodeType::File => {
            println!("File: size={:09} path={}", node.as_deref().unwrap().borrow().size, node.as_deref().unwrap().borrow().path);
        }
    }
    for child in node.as_deref().unwrap().borrow().children.iter() {
        print_nodes(child.clone(), level + 1);
    }
}

fn calculate_size(node: NodeLink) -> usize {
    match node.as_deref().unwrap().borrow().ntype {
        NodeType::File => {
            node.as_deref().unwrap().borrow().size
        },
        NodeType::Directory => {
            let mut size = 0;
            for c in node.as_deref().unwrap().borrow().children.iter() {
                c.as_ref().unwrap().borrow_mut().size = calculate_size(c.clone());
                size += c.as_deref().unwrap().borrow().size;
            }
            size
        },
    }
}

fn calculate_all(node: NodeLink) {
    let mut size = 0;
    for c in node.as_ref().unwrap().borrow_mut().children.iter() {
        c.as_ref().unwrap().borrow_mut().size = calculate_size(c.clone());
        size += c.as_deref().unwrap().borrow().size;
    }
    node.as_ref().unwrap().borrow_mut().size = size;
}

fn find_all_dirs(dirs: &mut Vec<NodeLink>, node: NodeLink, max: usize) {
    match node.as_deref().unwrap().borrow().ntype {
        NodeType::Directory => {
            if node.as_deref().unwrap().borrow().size <= max {
                dirs.push(node.clone());
            }
            for c in node.as_deref().unwrap().borrow().children.iter() {
                find_all_dirs(dirs, c.clone(), max);
            }
        },
        NodeType::File => {},
    }
}

fn find_child(node: NodeLink, path: String) -> NodeLink {
    match node.as_deref().unwrap().borrow().children.iter().find(|n| {
        n.as_deref().unwrap().borrow().path == path
    }) {
        Some(n) => n.clone(),
        None => {
           for c in node.as_deref().unwrap().borrow().children.iter() {
                find_child(c.clone(), path.clone());
           }
           None
        }
    }
}

fn populate_dirs(dirs: &mut Vec<NodeLink>, node: NodeLink) {
    match node.as_deref().unwrap().borrow().ntype {
        NodeType::Directory => {
            dirs.push(node.clone());
            for c in node.as_deref().unwrap().borrow().children.iter() {
                populate_dirs(dirs, c.clone());
            }
        },
        NodeType::File => {},
    }
}

fn get_size_from_dirs(dirs: &Vec<NodeLink>) -> usize {
    dirs.iter().map(|c| c.as_deref().unwrap().borrow().size).sum::<usize>()
}

fn find_smallest_dir_size(total_space: usize, used_space: usize, try_free: usize, dirs: &Vec<NodeLink>) -> usize {
    let free = total_space.abs_diff(used_space);
    let need_space = try_free.abs_diff(free);
    let mut found_dir_size = 0;

    for c in dirs.iter() {
        let s = c.as_deref().unwrap().borrow().size;
        if s >= need_space && (s <= found_dir_size || found_dir_size == 0) {
            found_dir_size = s;
        }
    }
    found_dir_size
}

fn main() {
    let contents = fs::read_to_string("./input7.txt").unwrap();

    /*let contents = "$ cd /
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
7214296 k
";*/

    let mut fs: Fs = Fs::default();
    fs.set_size(70000000);


    let mut dirs: Vec<NodeLink> = vec![];

    let file_r = Regex::new(r"^\d.*").unwrap();

    contents.lines().for_each(|l| {
        if l.starts_with("$ cd") {
            let n = l.strip_prefix("$ cd").unwrap().trim().to_string();
            if n == ".." {
                if fs.get_current().as_deref().unwrap().borrow().parent.is_some() {
                    fs.set_curr(fs.get_parent());
                }
            } else {
                if fs.get_current().is_none() {
                    let root = Node::new(NodeType::Directory, n.clone(), 0);
                    fs.set_root(Some(Rc::clone(&root)));
                    fs.set_curr(Some(root.clone()));
                } else {
                    let curr = Node::new(NodeType::Directory, n.clone(), 0);
                    fs.add_child(Some(Rc::clone(&curr)));
                    fs.set_curr(Some(Rc::clone(&curr)));
                }
            }
        } else if l.starts_with("$ ls") {

        } else {
            let m = file_r.find(l);
            match m {
                Some(ma) => {
                    let file = ma.as_str().split_once(" ").unwrap();
                    let node = Node::new(NodeType::File, file.1.to_string(), file.0.parse::<usize>().unwrap());
                    fs.add_child(Some(Rc::clone(&node))); 
                },
                None => {}
            }
        }
    });
    calculate_all(Some(Rc::clone(fs.get_root().as_ref().unwrap()))); // calculate size of all nodes
    fs.print_nodes();
    //find_all_dirs(&mut dirs, fs.get_root(), 100000); // populate with at most size
    //println!("{:#?}", get_size_from_dirs(&dirs));

    populate_dirs(&mut dirs, fs.get_root()); // populate with all dirs
    println!("{}", fs.get_root().as_deref().unwrap().borrow().size);
    //for i in dirs.iter() {
    //    println!("{}", i.as_deref().unwrap().borrow().size);
    //}
    println!("{:#?}", find_smallest_dir_size(fs.get_size(), fs.get_root().as_deref().unwrap().borrow().size, 30000000, &dirs));
}
