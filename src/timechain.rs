use crate::block::Block;

pub struct Timechain {
    pub chain: Vec<Block>,
}

impl Timechain {
    pub fn new() -> Self {
        Timechain {
            chain: vec![Self::create_genesis_block()],
        }
    }

    fn create_genesis_block() -> Block {
        let block = Block::new(0, 0, String::from("Bloque genesis"), String::from(""));

        block
    }

    pub fn get_latest_block(&self) -> &Block {
        &self.chain[self.chain.len()-1]
    }

    pub fn add_block(&mut self, mut new_block: Block) {
        let prev_block = self.get_latest_block();
        new_block.prev_hash = prev_block.hash.clone();
        new_block.hash = new_block.calculate_hash();
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let prev_block = &self.chain[i-1];
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
            if current_block.prev_hash != prev_block.hash {
                return false;
            }
        }
        true
    }
}
