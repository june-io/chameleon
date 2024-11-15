use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    error::Error,
    rc::Rc,
};

/// Struct representing each node of a Huffman Tree. Used to both
/// represent branches and leaves. Where branches are the inner
/// nodes, and leaves are the outer nodes holding values.
///
///          root
///         /    \
///       leaf   branch
///              /    \
///           leaf    leaf
///
/// FIX: Add examples.
pub struct Node {
    // Dictates whether the node is a leaf or a branch.
    // If true, the value is read, if false, the connected
    // nodes are read.
    leaf: bool,
    // The actual value stored which is irrelevant for branches.
    value: u8,
    // The frequency the of the value stored within the leaf.
    frequency: u32,
    // The code leading up to this node.
    address: u32,
    // The length of the address.
    length: usize,
    // The node to the left.
    left: Option<Rc<RefCell<Node>>>,
    // The node to the right.
    right: Option<Rc<RefCell<Node>>>,
}

// Allows for nodes to be compared by their frequency.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

// Allows for nodes to be partially compared.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Allows for nodes to be compared with the == operator.
impl Eq for Node {}

// Allows for node frequency to be compared with the == and != operators.
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency.eq(&other.frequency)
    }
}

// Implements the creation of new empty nodes.
impl Node {
    /// Creates a new empty HuffmanNode.
    ///
    /// # Returns
    ///
    /// A HuffmanNode with default values.
    ///
    /// FIX: Add examples.
    pub fn new() -> Self {
        Self {
            leaf: false,
            value: 0,
            frequency: 0,
            address: 0,
            length: 0,
            left: None,
            right: None,
        }
    }
}

// Creates a default for HuffmanNode.
impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

/// FIX: Document.
fn create_huffman_tree(frequencies: &[u32; 256]) -> ([Rc<RefCell<Node>>; 256], Rc<RefCell<Node>>) {
    // Create a leaf for every value that can be stored in a byte.
    let mut leaves: Vec<Rc<RefCell<Node>>> = (0..256)
        .map(|_| Rc::new(RefCell::new(Node::new())))
        .collect::<Vec<_>>();

    // Input nodes into a BinaryHeap
    // FIX: Learn more about BinaryHeaps, I only half know what this is doing.
    let mut nodes = BinaryHeap::new();

    // Iterate through each leaf and populate values before adding to the BinaryHeap.
    for (i, node_) in leaves.iter_mut().enumerate() {
        let mut node = node_.borrow_mut();
        node.leaf = true;
        node.value = i as u8;
        node.frequency = frequencies[i];

        // Drop the mutable borrow to free the node.
        drop(node);

        // Clone the smart pointer to the now mutated node.
        nodes.push(Reverse(node_.clone()));
    }

    // Loop intil only the root node is left.
    while nodes.len() > 1 {
        // .pop() returns the greatest item from the BinaryHeap,
        // and removes that item from the heap. Because the natural
        // order of the node is reversed 7 lines up, .pop() returns
        // the node with the lowest remaining frequency, which ensures
        // the least frequent values are at the bottom of the tree.
        // Unwrap can be called because the while loop ensures that
        // there is a node left and .pop() will only return None if
        // this is not true.
        let node_1 = nodes.pop().unwrap().0;
        let node_2 = nodes.pop().unwrap().0;

        // Creates the parent node.
        let parent = Node {
            leaf: false,
            value: 0,
            frequency: RefCell::borrow(&node_1)
                .frequency
                .saturating_add(RefCell::borrow(&node_2).frequency),
            address: 0,
            length: 0,
            left: Some(node_1.clone()),
            right: Some(node_2.clone()),
        };

        nodes.push(Reverse(Rc::new(RefCell::new(parent))));
    }

    // Once  more the while loop ensures .pop() will not return None.
    let root = nodes.pop().unwrap().0;

    // Create a vector to hold the nodes to calculate the address for.
    let mut queue = Vec::with_capacity(256);
    queue.push(root.clone());

    // Loop while the queue still has nodes in it.
    while let Some(node) = queue.pop() {
        let mut node = node.borrow_mut();

        // Check if the node is not a leaf, calculate the address of
        // it's branches.
        if !node.leaf {
            let left_option = node.left.as_ref();
            let right_option = node.right.as_ref();

            // Because the node is not marked as a leaf, it should have two child
            // nodes, however, if somehow this isn't the case, the node will be
            // assumed to be incorrectly labeled and will be treated as a leaf.
            let (left, right) = match (left_option, right_option) {
                (Some(left), Some(right)) => (left, right),
                (_, _) => {
                    eprintln!("Warning: Node not labeled as leaf has less than 2 children.");
                    node.leaf = true;
                    break;
                }
            };

            {
                let mut left_borrowed = left.borrow_mut();
                let mut right_borrowed = right.borrow_mut();
                left_borrowed.address = node.address << 1;
                left_borrowed.length = node.length + 1;
                right_borrowed.address = (node.address << 1) + 1;
                right_borrowed.length = node.length + 1;
            }

            queue.push(right.clone());
            queue.push(left.clone());
        }
    }

    // Panics only under extremely unforeseen circumstances.
    (
        leaves
            .try_into()
            .unwrap_or_else(|_| panic!("Error: Leaves could not be converted into array.")),
        root,
    )
}

/// FIX: Document.
pub struct Huffman {}

/// A trait for implementing encoding/decoding into structs.
///
/// # Types
///
/// * 'I' - Represents the type of the input.
/// * 'O' - Represents the type of the output.
/// * 'Error' - What error type should be returned in the Result.
///
/// # Methods
///
/// * 'encode' - Takes in an array of type I and returns a Result containing either the given
///             Error or a Vec of type O.
/// * 'decode' - Takes in an array of type O and returns a Result containing either the given
///             Error or a Vec of type I.
pub trait Coder<I: Copy, O: Copy> {
    type Error;
    fn encode(input: impl AsRef<[I]>) -> Result<Vec<O>, Self::Error>;
    fn decode(input: impl AsRef<[O]>) -> Result<Vec<I>, Self::Error>;
}

impl Coder<u8, u8> for Huffman {
    type Error = Box<dyn Error + Send + Sync + 'static>;
    /// Accepts a reference to an u8 array a encodes it into a
    /// Huffman Tree.
    fn encode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, Self::Error> {
        let input = input.as_ref();
        let mut output = Vec::new();
        let mut frequencies = [0u32; 256];

        // Count the occurances of each input byte.
        // FIX: Figure out how this works cause it does but I do not know why or how.
        input.iter().fold(&mut frequencies, |acc, &byte| {
            (*acc)[byte as usize] = (*acc)[byte as usize].saturating_add(1);
            acc
        });

        // Prepend counts/more iterator magic.
        frequencies
            .into_iter()
            .flat_map(|v| v.to_le_bytes())
            .for_each(|b| output.push(b));

        // The leaves are all thats needed for encoding so the root node is ignored.
        let (leaves, _) = create_huffman_tree(&frequencies);
        let mut next: u8 = 0;
        let mut filled = 0;

        // For each value in input, populate the tree with value.
        for &v in input.iter() {
            let leaf = RefCell::borrow(&leaves[v as usize]);
            let length = leaf.length;
            let mut code = leaf.address << (32 - length);

            // Output needs to be entire bytes.
            // FIX: It works but does not make sense.
            for _ in 0..length {
                // Set next equal to itself bitshifted left.
                next <<= 1;
                // If the most significant bit in a u32 is more than 0,
                // increment next
                if code & 0x8000_0000 > 0 {
                    next += 1;
                }
                filled += 1;
                code <<= 1;
                if filled == 8 {
                    output.push(next);
                    next = 0;
                    filled = 0;
                }
            }
        }

        if filled != 0 {
            next <<= 8 - filled;
            output.push(next);
        }
        Ok(output)
    }
    /// FIX: Document.
    fn decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, Self::Error> {
        let input = input.as_ref();
        if input.len() < std::mem::size_of::<u32>() * 256 {
            return Err(Self::Error::from("Error: Error decoding"));
        }
        let mut output = Vec::new();
        let mut count: u64 = 0;
        let freqs = input
            .iter()
            .take(std::mem::size_of::<u32>() * 256) // only read the byte counts
            .scan(([0u8; 4], 0usize), |s, b| {
                s.0[s.1] = *b;
                s.1 += 1;
                if s.1 == 4 {
                    s.1 = 0;
                    let num = u32::from_le_bytes(s.0);
                    count += num as u64;
                    Some(Some(num))
                } else {
                    Some(None)
                }
            })
            .flatten() // exclude all Nones
            .collect::<Vec<_>>();
        let input = &input[std::mem::size_of::<u32>() * 256..];
        let freqs: [u32; 256] = freqs
            .try_into()
            .map_err(|_| Self::Error::from("Error: Error decoding."))?;
        let (_, root) = create_huffman_tree(&freqs);
        let mut current: *const Node = root.as_ptr() as *const _;
        for &v in input {
            let mut v = v;
            for _ in 0..8 {
                let current_ = unsafe { &*current };
                if v & 0x80 == 0 {
                    let left = current_
                        .left
                        .as_ref()
                        .ok_or_else(|| Self::Error::from("Error: Error while decoding."))?
                        .as_ptr() as *const _;
                    current = left;
                } else {
                    let right = current_
                        .right
                        .as_ref()
                        .ok_or_else(|| Self::Error::from("Error: Error while decoding."))?
                        .as_ptr() as *const _;
                    current = right;
                }
                v <<= 1;

                let current_ = unsafe { &*current };
                if current_.leaf {
                    output.push(current_.value);
                    count -= 1;
                    if count == 0 {
                        return Ok(output);
                    }
                    current = root.as_ptr();
                }
            }
        }
        let _ = root;
        Ok(output)
    }
}
