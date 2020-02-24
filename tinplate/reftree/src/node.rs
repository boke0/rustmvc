pub struct Node<T>{
    sym: String,
    children: Vec<Node<T>>,
    val: Option<Box<T>>
}
impl<T> Node<T>{
    pub fn new(c: String) -> Node<T> {
        Node {
            sym: c,
            children: Vec::new(),
            val: Option::None
        }
    }
    pub fn apply(&mut self, mut s: Vec<&str>, val: Box<T>) {
        if s.len() == 0 {
            self.val=Option::Some(val);
            return;
        }
        let c=s.remove(0).to_string();
        for n in &mut self.children {
            if n.sym == c {
                n.apply(s,val);
                return;
            }
        }
        let mut new_node=Node::new(c);
        new_node.apply(s, val);
        self.children.push(
            new_node
        );
    }
    pub fn get_val(&self) -> &Option<Box<T>> {
        &self.val
    }
    pub fn next(&self, c: String) -> Option<&Node<T>> {
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
