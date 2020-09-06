use huffr::huffman_tree::*;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let pqueue = huff_queue_gen("test.txt")?;
    let tree = HuffTree::from_pqueue(pqueue);

    println!();
    println!();
    print!("[");
    tree.pre_traverse(|node| print!("{}, ", node.data.symbol));
    println!("]");

    let code_map = tree.get_code_map();

    println!("{:#?}", code_map);

    let file = File::open("test.txt")?;
    file.bytes()
        .map(|res| res.expect("error reading file"))
        .for_each(|b| {
            print!("{}", code_map.get(&b).expect("wrong input file"));
        });
    println!();
    Ok(())
}

fn huff_queue_gen(filename: &str) -> io::Result<BinaryHeap<HuffNode>> {
    let file = File::open(filename)?;
    let mut huff_map = [0; 256];

    file.bytes()
        .map(|res| res.expect("error reading file"))
        .for_each(|b| {
            huff_map[b as usize] += 1;
        });

    Ok(BinaryHeap::from(
        huff_map
            .iter()
            .enumerate()
            .map(|(index, count)| HuffNode {
                symbol: index.clone() as u8,
                freq: count.clone(),
            })
            .filter(|node| node.freq > 0)
            .collect::<Vec<HuffNode>>(),
    ))
}
