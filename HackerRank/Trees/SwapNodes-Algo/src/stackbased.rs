use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    data: i32,
    row: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    // Returns a reference counted internally mutable Node
    fn new(d: i32, r: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { data: d, row: r, left: None, right: None }))
    }

    // We use RefCell's swap function to swap the values of left and right
    fn swap_children(&mut self) {
        let l = self.left.as_mut().unwrap();
        let r = self.right.as_mut().unwrap();
        l.swap(r);
    }
}

// Should swap two node left and rights
fn _swap(row : &mut Vec<Rc<RefCell<Node>>>) {
    for i in 0..row.len() {
        // We clone our reference counted row[i] and get a mutable borrow
        let n = Rc::clone(&row[i]);
        let mut mn = n.borrow_mut();

        // Use the Node swap_children helper to actually do the swap
        mn.swap_children();
    }
}

fn traverse(node : &Rc<RefCell<Node>>, list : &mut Vec<i32>) {
    let n = node.borrow();
    match n.left {
        Some(ref x) => traverse(x, list),
        None => {}
    }

    list.push(n.data);

    match n.right {
        Some(ref x) => traverse(x, list),
        None => {}
    }
}

// The actual swap nodes function
pub fn swap_nodes(indexes : Vec<Vec<i32>>, queries : Vec<i32>) -> Vec<Vec<i32>> {
    let mut v = Vec::new();

    // We insert the first node which is always 1 in this situation at level 0
    v.push(Node::new(1, 0));

    let mut row = 0;

    // Our Vector that allows for quick access to the different nodes by index
    let mut qa = Vec::new();

    // Breadth first style queue push and pop to traverse the array
    while v.len() > 0 {
        // Pop and grab a mutable reference of the node in the queue
        let n = v.pop().unwrap();
        let mut nu = n.borrow_mut();

        // Grab the next left and rights from the array
        let l = indexes[row][0];
        let r = indexes[row][1];

        // If we aren't looking at tree leaf ends then create a new node
        if l != -1 {
            let ln = Node::new(l, nu.row + 1);
            v.push(Rc::clone(&ln));
            // Assign this as the left node of the above popped node from queue
            nu.left = Some(ln);
        }

        // Do the same for the node on the right
        if r != -1 {
            let rn = Node::new(r, nu.row + 1);
            v.push(Rc::clone(&rn));
            nu.right = Some(rn);
        }

        // Check to see if we need to create a new row of nodes
        if qa.len() < nu.row as usize + 1 {
            qa.push(Vec::new())
        }

        // Shove it into our quick access vector
        qa[nu.row as usize].push(Rc::clone(&n));
        row += 1;
    }

    let mut response :Vec<Vec<i32>> = Vec::new();

    // Start swapping the nodes according to the queries sent through
    for q in queries {
        for k in (q..qa.len() as i32).step_by(q as usize) {
            _swap(&mut qa[k as usize - 1]);
        }

        let mut inorder: Vec<i32> = Vec::new();

        // After each query we push the in-order traversal into our response
        traverse(&qa[0][0], &mut inorder);

        response.push(inorder);
    }

    return response;
}
