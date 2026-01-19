use utils;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::result;
use std::collections::{HashSet};


type NodeRef<T> = Rc<RefCell<TreeNode<T>>>;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Display,
{
    name: String,
    value: T,
    children: Vec<NodeRef<T>>,
    parents: Vec<NodeRef<T>>,
}

impl<T> TreeNode<T>
where
    T: Display,
{
    fn new(name: &str, value: T) -> NodeRef<T> {
        Rc::new(RefCell::new(TreeNode {
            name: name.to_string(),
            value,
            children: Vec::new(),
            parents: Vec::new(),
        }))
    }

    fn add_child(parent: &NodeRef<T>, child: &NodeRef<T>) {
        parent.borrow_mut().children.push(Rc::clone(child));
        child.borrow_mut().parents.push(Rc::clone(parent));
    }
}

fn main() {
    if let Ok(lines) = utils::read_lines("./input.txt") {
        let mut nodes: HashMap<String, NodeRef<String>> = HashMap::new();
        let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut root_node1: Option<NodeRef<String>> = None;
        let mut svr_node: Option<NodeRef<String>> = None;
        let mut fft_node: Option<NodeRef<String>> = None;
        let mut dac_node: Option<NodeRef<String>> = None;

        for line in lines.flatten() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut parts: Vec<&str> = line.split_ascii_whitespace().collect();
            let name = parts.remove(0).trim_end_matches(':').to_string();

            let node: Rc<RefCell<TreeNode<String>>> = TreeNode::new(&name, "".to_string());

            if name == "you" {                
                root_node1 = Some(Rc::clone(&node));
            }
            if name == "svr" {
                svr_node = Some(Rc::clone(&node));
            }
            if name == "dac" {
                dac_node = Some(Rc::clone(&node));
            }
            if name == "fft" {
                fft_node = Some(Rc::clone(&node));
            }

            nodes.insert(name.clone(), node);

            let children: Vec<String> = parts.into_iter().map(|s| s.to_string()).collect();
            children_map.insert(name, children);
        }

        for (name, children) in &children_map {
            let parent = nodes
                .get(name)
                .unwrap_or_else(|| panic!("Parent node {name} not found"));

            for child_name in children {
                if child_name == "out" {
                    continue;
                }
                let child = nodes
                    .get(child_name)
                    .unwrap_or_else(|| panic!("Child node {child_name} not found"));
                TreeNode::add_child(parent, child);
            }
        }

        let mut res1 = 0;
        if let Some(root) = &root_node1 {
            //println!("Root is: {:?}", root.borrow());
            res1 = one(root);
        }

        let mut res2 = 0;
        if let Some(root) = &svr_node {
            //println!("Root is: {:?}", root.borrow());
            res2 = two(root, &fft_node.unwrap(), &dac_node.unwrap(), &nodes);
        }
        
        println!("result 1: {} 2: {}", res1, res2);

    }
}

fn one(root: &NodeRef<String>) -> i64 {
    let mut result = 0;

    let mut todo: VecDeque<Vec<NodeRef<String>>> = VecDeque::new();
    todo.push_back(vec![Rc::clone(root)]);

    while let Some(path) = todo.pop_front() {
        let last = path.last().unwrap();
        let last_borrow = last.borrow();

        if last_borrow.children.is_empty() {
            // leaf path
            result += 1;
            continue;
        }

        for child in &last_borrow.children {
            // cycle in current path?
            if path.iter().any(|n| Rc::ptr_eq(n, child)) {
                panic!("circle detected involving {}", child.borrow().name);
            }

            let mut new_path = path.clone();
            new_path.push(Rc::clone(child));
            todo.push_back(new_path);
        }
    }

    result
}



fn mark_forward(start: &NodeRef<String>, mark: &mut HashSet<String>) {
    fn dfs(node: &NodeRef<String>, mark: &mut HashSet<String>) {
        let name = node.borrow().name.clone();
        if !mark.insert(name) {
            return;
        }
        let children = node.borrow().children.clone();
        for c in children {
            dfs(&c, mark);
        }
    }
    dfs(start, mark);
}

fn mark_backward(start: &NodeRef<String>, mark: &mut HashSet<String>) {
    fn dfs(node: &NodeRef<String>, mark: &mut HashSet<String>) {
        let name = node.borrow().name.clone();
        if !mark.insert(name) {
            return;
        }
        let parents = node.borrow().parents.clone();
        for p in parents {
            dfs(&p, mark);
        }
    }
    dfs(start, mark);
}

fn compute_is_sets(dac: &NodeRef<String>, fft: &NodeRef<String>) -> (HashSet<String>, HashSet<String>) {
    let mut is_dac = HashSet::new();
    let mut is_fft = HashSet::new();

    mark_forward(dac, &mut is_dac);
    mark_forward(fft, &mut is_fft);

    let dac_name = dac.borrow().name.clone();
    let fft_name = fft.borrow().name.clone();
    is_dac.remove(&dac_name);
    is_fft.remove(&fft_name);

    mark_backward(dac, &mut is_dac);
    mark_backward(fft, &mut is_fft);

    (is_dac, is_fft)
}

fn helper_two(
    parent_name: &str,
    nodes: &HashMap<String, NodeRef<String>>,
    is_dac: &HashSet<String>,
    is_fft: &HashSet<String>,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    if let Some(&cached) = cache.get(parent_name) {
        return cached;
    }

    let parent_node = nodes
        .get(parent_name)
        .unwrap_or_else(|| panic!("Unknown node {parent_name}"));

    let children = parent_node.borrow().children.clone();

    if children.len() == 0 {
        return 1;
    }
    let mut result = 0;

    for child in children {
        let child_name = child.borrow().name.clone();

        if !is_dac.contains(&child_name) {
            continue;
        }
        if !is_fft.contains(&child_name) {
            continue;
        }

        result += helper_two(&child_name, nodes, is_dac, is_fft, cache);
    }

    cache.insert(parent_name.to_string(), result);
    result
}

//490695961032000
/*We look forward and backward at all neighbors of fft and dac. 
Then we start at srv and see if the children are in both sets. 
If not, remove them; if already there, leave them. 
We cache a result, and if we end up at the same node again, we can stop.  */
fn two(svr: &NodeRef<String>, fft: &NodeRef<String>, dac: &NodeRef<String>, all_nodes: &HashMap<String, NodeRef<String>>) -> i64 {
    let (is_dac, is_fft) = compute_is_sets(dac, fft);
    let mut cache: HashMap<String, i64> = HashMap::new();
    
    helper_two(&svr.borrow().name, all_nodes, &is_dac, &is_fft, &mut cache)
}