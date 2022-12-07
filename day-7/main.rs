use std::cell::RefCell;
use std::rc::Rc;

mod node;
use node::*;

struct Terminal {
    pwd: Rc<RefCell<Node>>,
    root: Rc<RefCell<Node>>,
    folder_sizes: Vec<usize>,
}

impl Terminal {
    fn new(pwd: Rc<RefCell<Node>>, root: Rc<RefCell<Node>>) -> Self {
        Self {
            pwd,
            root,
            folder_sizes: vec![],
        }
    }

    fn cd(&mut self, name: &str) {
        match name {
            "/" => {
                self.pwd = Rc::clone(&self.root);
            }
            ".." => {
                self.pwd = self.parent();
            }
            _ => {
                if let Some(target) = self.get_child(name) {
                    self.pwd = target;
                }
            }
        }
    }

    fn fs_size_max(&mut self, max: usize) -> Vec<usize> {
        self.folder_sizes = vec![];

        // Calculate folder sizes
        self.calc_size_max(Rc::clone(&self.root), max);

        self.folder_sizes.clone()
    }

    fn calc_size_max(&mut self, node: Rc<RefCell<Node>>, max: usize) -> usize {
        let node = node.borrow();

        let size = match node.node_type {
            NodeType::File(size) => size,
            NodeType::Directory => {
                let sum = node
                    .children
                    .iter()
                    .map(|it| self.calc_size_max(Rc::clone(it), max.clone()))
                    .sum();

                // Append to count
                if sum <= max {
                    self.folder_sizes.push(sum);
                }

                sum
            }
        };

        size
    }

    fn get_child(&self, name: &str) -> Option<Rc<RefCell<Node>>> {
        let pwd = self.pwd.borrow();
        if let Some(target) = pwd.children.iter().find(|it| it.borrow().name == name) {
            return Some(Rc::clone(target));
        }

        None
    }

    fn parent(&self) -> Rc<RefCell<Node>> {
        let parent = match &self.pwd.borrow().parent {
            Some(val) => Rc::clone(&val),
            _ => Rc::clone(&self.pwd),
        };

        parent
    }

    fn ls(&self) {
        let current = self.pwd.borrow();
        let children = &current.children;

        if children.len() == 0 {
            println!("- Folder is empty -");
        }

        for item in children {
            let current = item.borrow();
            println!("{}", current);
        }
    }

    fn print_dir(&self) {
        let current = self.pwd.borrow_mut();
        println!("Current dir: {}", current.name);
    }

    fn parse_line(&mut self, line: &str) {
        // Command
        if line.starts_with('$') {
            let input: Vec<&str> = line.split(' ').skip(1).collect();
            let cmd = input.get(0).unwrap();
            let arg = input.get(1);

            match (*cmd, arg) {
                ("cd", Some(arg)) => {
                    self.cd(*arg);
                }
                _ => {}
            }
        } else {
            // Output
            let mut node: Node = line.into();
            node.set_parent(Rc::clone(&self.pwd));

            // Add child to current pwd
            let mut pwd = self.pwd.borrow_mut();
            pwd.add_child(Rc::new(RefCell::new(node)));
        }
    }
}

fn main() {
    let input = include_str!("input");

    let root = Rc::new(RefCell::new(Node::new(
        "/".to_string(),
        NodeType::Directory,
        None,
    )));

    let mut terminal = Terminal::new(Rc::clone(&root), Rc::clone(&root));

    for line in input.lines() {
        terminal.parse_line(line);
    }

    terminal.cd("/");

    // Part 1
    let total: usize = terminal.fs_size_max(100_000).iter().sum();
    println!("Sum of max 100k: {:?}", total);

    // Part 2
    let mut total = terminal.fs_size_max(usize::MAX);
    let root_size = total.pop().unwrap();
    let disk_size = 70_000_000;
    let free_size = disk_size - root_size;
    let desired_size = 30_000_000 - free_size;

    let mut target_folders: Vec<&usize> = total.iter().filter(|it| *it >= &desired_size).collect();
    target_folders.sort();
    println!("Delete dir with size: {:?}", target_folders.get(0).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root_folder() -> Rc<RefCell<Node>> {
        let root = Rc::new(RefCell::new(Node::new(
            "/".to_string(),
            NodeType::Directory,
            None,
        )));
        let a = Rc::new(RefCell::new(Node::new(
            "a".to_string(),
            NodeType::Directory,
            Some(Rc::clone(&root)),
        )));

        root.borrow_mut().add_child(Rc::clone(&a));

        root
    }

    #[test]
    fn parse_lines() {
        let root = root_folder();
        let mut terminal = Terminal::new(Rc::clone(&root), Rc::clone(&root));

        terminal.parse_line("$ cd /");
        terminal.parse_line("$ ls");
        terminal.parse_line("dir fchrtcbh");
        terminal.parse_line("57400 pfqcbp");
        terminal.parse_line("$ cd fchrtcbh");
        terminal.parse_line("$ ls");
        terminal.parse_line("61765 nlr");
        terminal.ls();

        assert_eq!(terminal.calc_size_max(root, usize::MAX), 119165);
    }

    #[test]
    fn example() {
        let input = r"$ cd /
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
dir y
$ cd y
$ ls
dir z
$ cd z
$ ls
1 zyx";
        let root = Rc::new(RefCell::new(Node::new(
            "/".to_string(),
            NodeType::Directory,
            None,
        )));
        let mut terminal = Terminal::new(Rc::clone(&root), Rc::clone(&root));

        terminal.cd("d");
        terminal.ls();

        for line in input.lines() {
            terminal.parse_line(line);
        }

        let result = terminal.fs_size_max(100_000);
        let sum = result.iter().sum::<usize>();
        assert_eq!(sum, 95439);
    }
}
