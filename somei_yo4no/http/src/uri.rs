use std::collections::HashMap;

enum State {
    Path,
    QueryKey,
    QueryVal,
    Fragment
}
#[derive(Debug,PartialEq)]
pub struct Uri {
    pub base: Vec<u8>,
    pub path: String,
    pub query: HashMap<String, String>,
    pub fragment: String
}
impl Uri {
    pub fn from(uri: String) -> Uri {
        let mut path = String::new();
        let mut query = HashMap::new();
        let mut fragment = String::new();
        let mut key_buf = String::new();
        let mut val_buf = String::new();
        let mut state = State::Path;
        for c in uri.chars() {
            match c {
                '?' => state = State::QueryKey,
                '=' => state = State::QueryVal,
                '&' => {
                    state = State::QueryKey;
                    query.insert(key_buf, val_buf);
                    key_buf=String::new();
                    val_buf=String::new();
                },
                '#' => {
                    state = State::Fragment;
                    query.insert(key_buf, val_buf);
                    key_buf=String::new();
                    val_buf=String::new();
                },
                _ => match state {
                    State::Path => path.push(c),
                    State::QueryKey=> key_buf.push(c),
                    State::QueryVal => val_buf.push(c),
                    State::Fragment => fragment.push(c)
                }
            }
        }
        Uri {
            base: Vec::new(),
            path: path,
            query: query,
            fragment: fragment
        }
    }
    pub fn to_string(&mut self) -> String {
        let mut str = String::new();
        str.push_str(self.path.as_str());
        str.push('?');
        let mut iter = self.query.iter();
        match iter.next() {
            Some((k,v)) => {
                let mut k_=k;
                let mut v_=v;
                loop {
                    str.push_str(k_.as_str());
                    str.push('=');
                    str.push_str(v_.as_str());
                    match iter.next() {
                        Some((k,v)) => {
                            str.push('&');
                            k_=k;
                            v_=v;
                        },
                        None => break
                    }
                }
            },
            None => {}
        }
        if self.fragment.len() > 0 {
            str.push('#');
            str.push_str(self.fragment.as_str());
        }
        str
    }
}
