mod node;

pub struct Tree<T>{
    root: node::Node<T>
}
impl<T> Tree<T>{
    pub fn new () -> Tree<T> {
        let r=node::Node::new(String::from(""));
        Tree {
            root: r
        }
    }
    pub fn apply(&mut self, s: Vec<&str>, val: Box<T>) {
        self.root.apply(s, val);
    }
    pub fn get(&self,s: Vec<&str>) -> &Option<Box<T>>{
        let mut n = &self.root;
        for c in s {
            n = &mut n.next(c.to_string()).unwrap();
        }
        n.get_val()
    }
}
