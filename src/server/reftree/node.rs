pub struct Node {
    sym: String,
    children: Vec<Node>,
    id: Option<i16>
}
impl Node {
    pub fn new (c: String) -> Node {
        Node {
            sym: c,
            children: Vec::new(),
            id: Option::None
        }
    }
    pub fn apply (&mut self, mut s: Vec<&str>, id: i16) {
        if s.len() == 0 {
            println!("id: {}",id);
            self.id=Option::Some(id);
            return;
        }
        let c=s.remove(0).to_string();
        for n in &mut self.children {
            if n.sym == c {
                n.apply(s,id);
                return;
            }
        }
        let mut new_node=Node::new(c);
        new_node.apply(s, id);
        self.children.push(
            new_node
        );
    }
    pub fn get_val(&self) -> Option<i16> {
        self.id
    }
    pub fn next (&self, c: String) -> Option<&Node> {
        for n in &self.children {
            if n.sym == c {
                return Some(
                    &n
                );
            }
        }
        None
    }
}
