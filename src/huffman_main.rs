use huffman::{create_huffman_tree, decode, encode, HuffmanEncoding};

mod huffman;

fn main() {
    let text = "This is test data for generating a huffman encoding!";
    let huffman_tree = create_huffman_tree(text).unwrap();
    println!("{:#?}", huffman_tree);
    let huffman_encoding = HuffmanEncoding::new(&huffman_tree);
    println!("{:#?}", huffman_encoding);
    let text2 = "This is test data for encoding and decoding!";
    let encoded_data = encode(&huffman_encoding, text2);
    println!("{:#?}", encoded_data);
    let decoded_text = decode(&huffman_encoding, &encoded_data);
    println!("{}", decoded_text);
    assert_eq!(text2, decoded_text);
}
