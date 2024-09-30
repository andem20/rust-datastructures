use binary_tree::binary_tree::BinaryTree;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let array: Vec<u8> = (0..10).into_iter().map(|_i| rng.gen::<u8>()).collect();

    let binary_tree = BinaryTree::from_array(&array);

    println!("{:?}", array);
    println!("\n\n");
    println!("{}", binary_tree);
}
