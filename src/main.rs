mod lib;
use crate::lib::{Btree, BtreeNode};

fn main() {
    let mut t = Btree {
        root: BtreeNode::new(3, true),
        t: 3,
    }; // A B-Tree with minium degree 3
    t.insert(10);
    t.insert(20);
    t.insert(5);
    t.insert(6);
    t.insert(12);
    t.insert(30);
    t.insert(7);
    t.insert(17);
    t.insert(11);
    t.insert(21);
    t.insert(51);
    t.insert(61);
    t.insert(121);
    t.insert(31);
    t.insert(71);
    t.insert(171);
    t.insert(13);
    t.insert(22);
    t.insert(52);
    t.insert(62);
    t.insert(122);
    t.insert(322);
    t.insert(72);
    t.insert(172);
    println!("{:?}", t.root);
    t.root.transverse();
}
