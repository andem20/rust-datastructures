use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> BinaryTree<T> {
    pub fn from_array(array: &[T]) -> BinaryTree<T>
    where
        T: Copy + std::cmp::PartialOrd,
    {
        let mut binary_tree = BinaryTree { root: None };

        array
            .iter()
            .for_each(|value| Self::insert_node_recursion(&mut binary_tree.root, value));

        return binary_tree;
    }

    fn insert_node_recursion(current: &mut Option<Box<Node<T>>>, value: &T)
    where
        T: Copy + std::cmp::PartialOrd,
    {
        match current {
            Some(node) => {
                if &node.value > &value {
                    Self::insert_node_recursion(&mut node.left, value);
                } else {
                    Self::insert_node_recursion(&mut node.right, value);
                }
            }
            None => {
                let new_node = Node {
                    value: *value,
                    left: None,
                    right: None,
                };

                *current = Some(Box::new(new_node));
            }
        }
    }

    fn inorder_traverse<'a>(current: Option<&'a Box<Node<T>>>, mut result: &mut Vec<&'a T>) {
        if let Some(node) = current {
            Self::inorder_traverse(node.left.as_ref(), &mut result);
            result.push(&node.value);
            Self::inorder_traverse(node.right.as_ref(), &mut result);
        }
    }

    #[allow(dead_code)]
    fn traverse<'a>(current: Option<&'a Box<Node<T>>>, mut result: &mut Vec<&'a T>) {
        if let Some(node) = current {
            result.push(&node.value);
            Self::inorder_traverse(node.left.as_ref(), &mut result);
            Self::inorder_traverse(node.right.as_ref(), &mut result);
        }
    }

    fn breadth_first_traversal<'a>(
        current: Option<&'a Box<Node<T>>>,
        result: &mut Vec<Option<&'a T>>,
    ) where
        T: Debug,
    {
        let mut queue: Vec<Option<&Box<Node<T>>>> = Vec::new();

        queue.insert(0, current);

        let max_depth = 2_i32.pow(6);
        let mut i = 0;

        while let Some(element) = queue.pop() {
            if i >= max_depth - 1 {
                break;
            }

            let value: Option<&T> = match element {
                Some(node) => {
                    queue.insert(0, node.left.as_ref());
                    queue.insert(0, node.right.as_ref());
                    Some(&node.value)
                }
                None => {
                    queue.insert(0, None);
                    queue.insert(0, None);
                    None
                }
            };

            result.push(value);

            i += 1;
        }
    }

    pub fn inorder_traversal(&self) -> Vec<&T> {
        let mut result: Vec<&T> = Vec::new();

        Self::inorder_traverse(self.root.as_ref(), &mut result);

        return result;
    }
}

impl<T: std::fmt::Debug + Display> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        let max_depth = 6;
        let fill = " ";
        let width = 3;
        let last_row_amount = 2_usize.pow((max_depth - 1) as u32);
        let max_width = last_row_amount * width + last_row_amount;

        let mut result: Vec<Option<&T>> = Vec::new();

        Self::breadth_first_traversal(self.root.as_ref(), &mut result);

        let mut i = 0;
        let mut x = 1;
        let mut depth = 0;

        while i < result.len() {
            let start = i;
            let end = (x + i).min(result.len());

            let amount = 2_usize.pow(depth as u32);
            let spacing = (max_width / amount) - width;
            println!();
            for (j, num) in result[start..end].iter().enumerate() {
                let mut space = fill.repeat(spacing);
                if j == 0 {
                    space = fill.repeat(spacing / 2);
                }

                match num {
                    Some(n) => print!("{space}{n:0width$}"),
                    None => print!("{space}{}", "_".repeat(width)),
                }
            }

            println!();
            if depth != max_depth - 1 {
                let amount = 2_usize.pow(depth as u32);
                let spacing = (max_width / amount) - (width + 2);
                for j in 1..=x {
                    let mut space = fill.repeat(spacing);
                    if j == 1 {
                        space = fill.repeat(spacing / 2);
                    }

                    print!("{space}/{whitespace:0width$}\\", whitespace = " ");
                }
            }

            println!();

            i += x;
            x *= 2;
            depth += 1;
        }

        Ok(())
    }
}

//
//             12
//            /  \
//           3   22
//          / \
//            4
//           / \
//              5
//             / \
//                9
//
