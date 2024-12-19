mod huffman;

fn main() {
    let text = "this is an example for huffman encoding";
    let huffman_tree = huffman::create_huffman_encoding(text).unwrap();
    println!("{:#?}", huffman_tree);
}
