extern crate stdweb;

use stdweb::prelude::*;

pub trait Component {
    fn new() -> Self;
    fn update(&mut self, event: &Event);
    fn view(&self) -> Template;
    fn tagname(&self) -> &str;
    fn get_attr_fields(&self) -> Vec<&str>;
    fn construct() {
        
    }
    fn register() {
        js!{
            window.customElements.define(
                @{T::get_tagname()},
                class extends HTMLElement {
                    constructor() {
                        super();
                        this.model = (@{T::new})();
                        this.shadow = this.attachShadow({mode:"closed"});
                        (@{T::view})();
                    }
                }
            );
        }
    }
}
pub trait Event {
    fn get_message(&self);
}
pub struct Attr {
    hashmap: HashMap<String, String>
}
impl Attr {
    pub fn get(&self, k: &str) -> &str {
        return self.hashmap.get(k.to_string());
    }
    pub fn set(&mut self, k: &str, v: &str) {
        self.hashmap.insert(k.to_string(),v.to_string());
    }
}
