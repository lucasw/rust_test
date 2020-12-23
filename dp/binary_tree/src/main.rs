
struct Leaf<'a> {
    val : i64,
    left : Option<&'a Leaf<'a>>,
    right : Option<&'a Leaf<'a>>,
}

// TODO(lucasw) depth and breadth
fn print_leaf_optional(leaf : Option<&Leaf>) {
    match leaf {
        Some(x) => {
            println!("val {}", x.val);
            print_leaf_optional(x.left);
            print_leaf_optional(x.right);
        }
        None => (),
    }
}

fn print_leaf(leaf : &Leaf) {
    print_leaf_optional(Some(&leaf))
}

fn main() {
    // construct the tree from the bottom up
    let l0 = Leaf {
        val: 0,
        left: None,
        right: None,
    };

    let l1 = Leaf {
        val: 1,
        left: None,
        right: None,
    };

    let l2 = Leaf {
        val: 2,
        left: Some(&l0),
        right: Some(&l1),
    };

    print_leaf(&l2);
}
