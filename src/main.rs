mod rope;

use rope::Rope;

fn main() {
    let rope1 = Rope::new_leaf("Hello, ");
    let rope2 = Rope::new_leaf("World!");

    let rope = rope1.concatenate_ropes(rope2);

    println!("Rope length: {}", rope.node_length());
    println!("Substring (0, 5): {}", rope.get_a_substring(0, 5));

    let (left, right) = rope.split_rope(3);
    println!("Left part after split: {:?}", left);
    println!("Right part after split: {:?}", right);
}
