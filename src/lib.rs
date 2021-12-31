#![warn(clippy::all)]
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::convert::TryInto;

#[cfg(test)]
mod tests {
    use crate::Generator;
    #[test]
    fn common() {
        let mut g = Generator::default();
        g.train("some stupid words to test some stupid code");
        assert!(g.generate(100).is_some())
    }
    #[test]
    fn empty() {
        let mut g = Generator::default();
        g.train("");
        eprintln!("{:?}", g);
        let out = g.generate(20);
        assert_eq!(out, None)
    }
    #[test]
    fn contain_test() {
        let mut g = Generator::default();
        let text = "s 1 2 3 e\n\
                    s 2 3 4 e";
        g.train(text);
        eprintln!("{:?}", g);
        for _ in 1..100 {
            let default = String::from("not contains");
            let generated = g.generate(20).unwrap_or(default);
            if text.contains(&generated) {
                panic!("Contains check failed!")
            }
        }
    }
}
type Node = Option<Box<str>>;
type Key = [Node; 2];

#[derive(Debug, Clone)]
pub enum Limit {
    Unlimited,
    Limited { min: usize, overflow: usize },
}
impl Default for Limit {
    fn default() -> Self {
        Self::Unlimited
    }
}

#[derive(Default, Debug)]
pub struct Generator {
    limit: Limit,
    text: Vec<String>,
    chain: HashMap<Key, Vec<Node>>,
}
impl Generator {
    pub fn new(limit: Limit) -> Self {
        Self {
            limit,
            text: Vec::new(),
            chain: HashMap::new(),
        }
    }
    pub fn train(&mut self, text: &str) {
        self.text.push(String::from(text));
        self.update_chain(text);
        self.rebuild();
    }
    fn update_chain(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        if text.contains('\n') {
            for s in text.split('\n') {
                self.train(s)
            }
            return;
        }
        let mut text = text
            .split_whitespace()
            .map(String::from)
            .map(String::into_boxed_str)
            .map(Option::from)
            .collect::<Vec<_>>();
        text.insert(0, None);
        text.insert(0, None);
        text.push(None);
        for window in text.windows(3) {
            let (key, value) = window.split_at(2);
            match self.chain.get_mut(key) {
                Some(vector) => vector.extend_from_slice(value),
                None => {
                    assert!(
                        key.len() == 2,
                        "Something is wrong with std: slice len != 2"
                    );
                    let key = key.to_owned().try_into().unwrap();
                    self.chain.insert(key, value.to_vec());
                }
            };
        }
    }
    pub fn generate(&self, tries: usize) -> Option<String> {
        if tries == 0 {
            return None;
        }
        let mut rng = rand::thread_rng();
        let mut string: Vec<Node> = vec![None, None];
        loop {
            let index = &string[string.len() - 2..];
            let variants = self.chain.get(index)?;
            let choice = variants.choose(&mut rng)?.clone();
            if choice.is_none() {
                break;
            }
            string.push(choice);
        }
        let result = string
            .into_iter()
            .skip(2)
            .map(Option::unwrap) //We skipped first 2 Nones, so it's safe
            .collect::<Vec<_>>()
            .join(" ");
        if !self.text.contains(&result) {
            Some(result)
        } else {
            self.generate(tries - 1)
        }
    }
    fn rebuild(&mut self) {
        if !self.need_rebuild() {
            return;
        }
        if let Limit::Limited { min, .. } = self.limit {
            let len = self.text.len();
            let tail_of_text = &self.text[len - min..];
            let mut new_self = Self::new(self.limit.clone());
            for message in tail_of_text {
                new_self.update_chain(message)
            }
            new_self.text = tail_of_text.to_vec();
            *self = new_self
        }
    }
    fn need_rebuild(&self) -> bool {
        matches!(self.limit, Limit::Limited { min, overflow } if self.text.len() > min + overflow)
    }
}
#[cfg(test)]
#[test]
fn check_wrapping() {
    let mut generator = Generator::new(Limit::Limited {
        min: 2,
        overflow: 1,
    });
    for _ in 0..3 {
        generator.train("first")
    }
    assert_eq!(generator.text, &["first"; 3]);
    generator.train("second");
    assert_eq!(generator.text, &["first", "second"]);
}
