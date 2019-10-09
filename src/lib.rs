#[derive(Clone, Debug)]
pub struct BtreeNode {
  keys: Vec<isize>,
  t: usize,
  pub children: Vec<Option<Box<BtreeNode>>>,
  leaf: bool,
  n: usize,
}

pub struct Btree {
  pub root: Box<BtreeNode>,
  pub t: usize,
}

impl BtreeNode {
  pub fn new(t: usize, leaf: bool) -> Box<BtreeNode> {
    let mut keys = Vec::new();
    keys.resize(2 * t, 0);
    let mut children: Vec<Option<Box<BtreeNode>>> = Vec::new();
    children.resize(2 * t + 1, None);
    Box::new(BtreeNode {
      keys: keys,
      t: t,
      children: children,
      leaf: leaf,
      n: 0,
    })
  }
  pub fn transverse(&self) {
    let mut backup = 0;
    for i in 0..self.n {
      // println!("{}, {:?}", self.n, self.keys);
      if self.leaf == false {
        self.children[i].as_ref().unwrap().transverse();
      }
      println!("{}", &self.keys[i]);
      backup = i;
    }
    if self.leaf == false {
      self.children[backup + 1].as_ref().unwrap().transverse();
    }
  }
  pub fn search(&self, k: isize) -> Option<&BtreeNode> {
    let mut i = 0;
    while i < self.n && self.keys[i] < k {
      i += 1;
    }
    if self.keys[i] == k {
      return Some(&self);
    }
    if self.leaf == true {
      return None;
    }
    self.children[i].as_ref().unwrap().search(k)
  }
  pub fn split_child(&mut self, key: usize) {
    let old_node = &mut self.children[key as usize];
    let mut new_node = BtreeNode::new(self.t, old_node.as_ref().unwrap().leaf);
    new_node.n = self.t - 1;
    for j in 0..new_node.t - 1 {
      new_node.keys[j] = old_node.as_ref().unwrap().keys[j + new_node.t];
      old_node.as_mut().unwrap().keys[j + new_node.t] = 0;
    }
    if old_node.as_ref().unwrap().leaf == false {
      for j in 0..new_node.t {
        new_node.children[j] = Some(Box::new(
          ((old_node.as_ref().unwrap().children[j + new_node.t])
            .as_ref()
            .unwrap()
            .as_ref())
          .clone(),
        ));
        old_node.as_mut().unwrap().children[j + new_node.t] = None;
      }
    }
    old_node.as_mut().unwrap().n = self.t - 1;
    let pushed_key = old_node.as_ref().unwrap().keys[self.t - 1];
    for j in self.n..(key + 1) {
      self.children[j + 1] = Some((self.children[j].as_ref().unwrap()).clone());
      // println!("{:?}, {}", self.children[j + 1], j);
    }
    self.children[key + 1] = Some(new_node);
    // println!("{:?}", self.children[key + 1]);
    for j in self.n..=key {
      self.keys[j + 1] = self.keys[j];
    }
    self.keys[key] = pushed_key;
    self.n += 1;
  }
  pub fn insert_non_full(&mut self, k: isize) {
    let mut i: isize = self.n as isize - 1;
    // println!("{}", self.n);
    if self.leaf == true {
      while i >= 0 && self.keys[i as usize] > k {
        self.keys[(i + 1) as usize] = self.keys[i as usize];
        i -= 1;
      }
      self.keys[(i + 1) as usize] = k;
      self.n += 1;
    } else {
      while i >= 0 && self.keys[i as usize] > k {
        i -= 1;
      }
      if self.children[(i + 1) as usize].as_ref().unwrap().n == 2 * self.t - 1 {
        self.split_child((i + 1) as usize);
        if self.keys[(i + 1) as usize] < k {
          i += 1;
        }
      }
      self.children[(i + 1) as usize]
        .as_mut()
        .unwrap()
        .insert_non_full(k);
    }
  }
}

impl Btree {
  pub fn insert(&mut self, k: isize) {
    if self.root.n == 2 * self.t - 1 {
      let mut new_node = BtreeNode::new(self.t, false);
      let old_root = (*self.root).clone();
      new_node.children[0] = Some(Box::new(old_root));
      self.root = new_node;
      self.root.split_child(0);
      let mut i = 0;
      if self.root.keys[0] < k {
        i += 1;
      }
      self.root.children[i].as_mut().unwrap().insert_non_full(k);
    } else {
      self.root.insert_non_full(k);
    }
  }
}
