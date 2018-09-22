// In order traversal of the array based tree
fn traverse(v: &mut Vec<i32>, left: &Vec<i32>, right: &Vec<i32>, node: i32) {
    if node == -1 {
        return;
    }
    let n = node as usize;

    traverse(v, &left, &right, left[n]);
    v.push(node);
    traverse(v, &left, &right, right[n]);
}

fn swap(left: &mut Vec<i32>, right: &mut Vec<i32>, q: i32, node: i32) {
    // If this node is a -1 then do nothing
    if node == -1 {
        return;
    }

    // Rust slice/array indexes are usize, can't use i32
    let n = node as usize;

    // If the current q (K) is a multiple of node then swap
    // This works because we have 0 in both left[0] and left[1]
    // as padding for both arrays which alleviates having to
    // perform left[n - 1] right[n - 1] tedius operations
    if q % node == 0 {
        let t = left[n];
        left[n] = right[n];
        right[n] = t;
    }

    // We do this as we don't want to have a mutable and immutable borrow below when swapping.
    /*
        In a tree of:
             1
          2     3
        -1 -1 -1 -1

        If node == 1 then the below left_node = 2, right_node = 3
        Array indexes are as follows:
                           null
                left[1]           right[1]
            left[2] right[2]  left[3]  right[3]
    */

    let left_node = left[n];
    let right_node = right[n];

    /*
        From above, swap is called with q + 1 and the left_node which is 2.
        This then allows the actual swap above to swap according to left[2] and right[2]
        illustrated in the above crude tree.

        The same is then performed for the right_node = 2 == left[3] and right[3]

        Effectively doing a breadth-first search without having to construct
        classes/structs with pointers betwen them
    */
    // Recursively swap for each q
    swap(left, right, q + 1, left_node);
    swap(left, right, q + 1, right_node);
}

pub fn swap_nodes(indexes : Vec<Vec<i32>>, queries : Vec<i32>) -> Vec<Vec<i32>> {
    // Create the two node arrays
    let mut left = Vec::new();
    let mut right = Vec::new();

    // We push dummy numbers to pad the arrays so that our K queries don't have to be "n-1" indexed later
    left.push(0);
    right.push(0);

    // Toss each pair of numbers into one or the other
    for i in indexes {
        // left ---->  2  3 <---- right
        // left ----> -1 -1 <---- right
        left.push(i[0]);
        right.push(i[1]);
    }

    // Go through each of the queries
    let mut response = Vec::new();
    for q in queries {
        // Our initial node is 1
        let initial_node = 1;
        // execute the swap based on our query K represented here by Q
        swap(&mut left, &mut right, q, initial_node);

        // Then traverse through the tree
        let mut row = Vec::new();
        traverse(&mut row, &left, &right, 1);

        // Add our traversal into our final vector
        response.push(row);
    }

    return response;
}
