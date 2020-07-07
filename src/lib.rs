use std::collections::HashMap;
use rand::seq::SliceRandom;

#[cfg(test)]
mod tests {
    use crate::Generator;
    #[test]
    fn test() {
        let mut g = Generator::new();
        g.train("some stupid words to test some stupid code");
        println!("{}",g.generate(20))
    }
}
type Node = Option<String>;
type Key = Vec<Node>; //len should always be 2
pub struct Generator{
    text: String,
    chain: HashMap<Key, Vec<Node>>
}
impl Generator{
    pub fn new() -> Self {
        Generator {
            text: String::new(),
            chain: HashMap::new()
        }
    }
    pub fn train(&mut self, text: &str){
        if text.is_empty(){
            return
        }
        self.text.push_str(text);
        let mut text = text.split_whitespace()
            .map(String::from)
            .map(Option::from)
            .collect::<Vec<_>>();
        text.insert(0, None);
        text.insert(0, None);
        text.push(None);
        for window in text.windows(3) {
            //let window = window.to_owned();
            let (key, value) = window.split_at(2);
            if self.chain.contains_key(key) {
                self.chain.get_mut(key).unwrap().push(value[0].clone());
            }
            else {
                self.chain.insert(key.to_vec(), value.to_vec()); 
            }
        }
        
    }
    pub fn generate(&self, tries:usize) -> String {
        if tries == 0 {
            return String::from("хуй тебе");
        }
        let mut rng = rand::thread_rng();
        let mut string: Vec<Node> = vec![None, None];
        loop {
            let index = &string[string.len()-2..];
            let variants = self.chain.get(index).unwrap();
            let choice = variants.choose(&mut rng).unwrap().clone();
            if choice.is_none() {
                break
            }
            string.push(choice);
        }
        let result = string.into_iter()
            .skip(2)
            .map( Option::unwrap )
            .collect::<Vec<_>>()
            .join(" ");
        if !self.text.contains(&result) {
            result
        }
        else {
            self.generate(tries-1)
        }
    }
}
