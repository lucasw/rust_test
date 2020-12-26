/*
 * Create a directed cyclic graph and traverse it, list all the nodes once.
 *
 *
 *
 */

use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

// TODO(lucasw) do a version of this without the Arc Mutex, just store uuids of the children.
struct Node {
    name : String,
    uuid : Uuid,
    children : Vec<Arc<Mutex<Node>>>,
}

fn print_nodes(node_list : &mut HashSet<Uuid>, node_arc : &Arc<Mutex<Node>>)
{
    let children;
    {
        let node = &*node_arc.lock().unwrap();
        if node_list.contains(&node.uuid) {
            return;
        }
        node_list.insert(node.uuid);
        children = node.children.clone();
        println!("{} uuid {}, num children {}", node.name, node.uuid, node.children.len());
    }
    for child in children.iter() {
        let node = &*child.lock().unwrap();
        print!("{} ", node.name);
    }
    println!("");
    // TODO(lucasw) do a breadth first version of this
    for child in children.iter() {
        print_nodes(node_list, child);
    }
}

fn main() {

    let mut root = Node {
        name: "root".to_string(),
        uuid : Uuid::new_v4(),
        children : Vec::new(),
    };
    let root_arc = Arc::new(Mutex::new(root));

    {
        let mut node1 = Node {
            name: "n1".to_string(),
            uuid : Uuid::new_v4(),
            children : Vec::new(),
        };
        let n1_arc = Arc::new(Mutex::new(node1));

        let mut node2 = Node {
            name: "n2".to_string(),
            uuid : Uuid::new_v4(),
            children : Vec::new(),
        };

        let mut node3 = Node {
            name: "n3".to_string(),
            uuid : Uuid::new_v4(),
            children : Vec::new(),
        };
        let n3_arc = Arc::new(Mutex::new(node3));

        (*root_arc.lock().unwrap()).children.push(n1_arc.clone());
        (*root_arc.lock().unwrap()).children.push(Arc::new(Mutex::new(node2)));
        (*n1_arc.lock().unwrap()).children.push(n3_arc.clone());
        // loop
        (*n3_arc.lock().unwrap()).children.push(root_arc.clone());
    }
    // after leaving the above scope, root should be the only handle we have into all the other
    // nodes
    let mut node_list = HashSet::new();
    print_nodes(&mut node_list, &root_arc);
    print_nodes(&mut node_list, &root_arc);
}
