use coin::block::Block;
use coin::timechain::Timechain;

fn main() {
    let b1 = Block::new(1, 121212, String::from("bloque 1"), String::from("0x00"));
    let b2 = Block::new(2, 1122, String::from("bloque 2"), String::from("0x00"));
    let mut tc = Timechain::new();
    tc.add_block(b1);
    tc.add_block(b2);
    for block in tc.chain.iter() {
        println!("{}", block);
    }
    tc.chain[1].index = 3;
    println!("Chain is valid? {}", tc.is_valid());
}
