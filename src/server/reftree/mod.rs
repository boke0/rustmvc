mod node;

pub struct Tree {
    root: node::Node
}
impl Tree {
    pub fn new () -> Tree {
        let r=node::Node::new(String::from(""));
        Tree {
            root: r
        }
    }
    pub fn apply(&mut self, s: Vec<&str>, id: i16) {
        self.root.apply(s, id);
    }
    pub fn get(&self,s: Vec<&str>) -> Option<i16>{
        let mut n = &self.root;
        for c in s {
            n = &mut n.next(c.to_string()).unwrap();
        }
        n.get_val()
    }
}
