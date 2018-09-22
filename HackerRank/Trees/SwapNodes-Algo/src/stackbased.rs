use std::mem::swap;

struct Node {
    data: i32,
    row: i32,
    left: Option<Box<Node>>, // Box<T> are "pointers"
    right: Option<Box<Node>>,
}

impl Node {
    fn new(d: i32, r: i32) -> Node {
        Node { data: d, row: r, left: None, right: None }
    }
}

fn _swap(row : &mut Vec<Box<Node>>) {
    for i in 0..row.len() {
        let n = &mut row[i];
        let (mut left, mut right) = (&n.left, &n.right);
        swap(&mut left, &mut right);
        // let t = &row[i].left;
        // row[i].left = row[i].right;
        // row[i].right = *t;
    }
}

pub fn swap_nodes(indexes : Vec<Vec<i32>>, queries : Vec<i32>) -> Vec<Vec<i32>> {
    let mut v :Vec<Box<&Node>> = Vec::new();
    v.push(Box::new(&Node::new(1, 0)));

    let mut row = 0;

    let mut qa: Vec<Vec<Box<Node>>> = Vec::new();

    while v.len() > 0 {
        let n = v.pop();

        let l = indexes[row][0];
        let r = indexes[row][1];

        let mut nu = n.unwrap();

        if l != -1 {
            let ln = Box::new(&Node::new(l, nu.row + 1));
            v.push(ln);
            nu.left = Some(ln);
        }

        if r != -1 {
            let rn = Box::new(&Node::new(r, nu.row + 1));
            v.push(rn);
            nu.right = Some(rn);
        }

        if qa.len() < nu.row as usize + 1 {
            qa.push(Vec::new())
        }

        qa[nu.row as usize].push(nu);
        row += 1;
    }

    let mut response :Vec<Vec<i32>> = Vec::new();

    for q in queries {
        for k in (q..qa.len() as i32).step_by(q as usize) {
            _swap(&mut qa[k as usize]);
        }

        let mut inorder: Vec<i32> = Vec::new();

        response.push(inorder);
    }

    return response;
}
