fn main() {
    println!("Hello, world!");

    use self::BinaryTree::*;
    let mars_tree = NonEmpty(Box::new(TreeNode{
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

