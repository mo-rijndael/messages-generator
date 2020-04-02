use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Generator{
    text: String,
    chain: HashMap<(String, String), Vec<String>>
}
impl Generator{
    pub fn train(&mut self, text: &str){
        if text.is_empty(){
            return
        }
        let mut vector = vec![String::new();2];
        for i in text.split_whitespace(){
            vector.push(i.to_string());
        }
        for i in 0..vector.len()-2{
            let  key = (vector[i].clone(), vector[i+1].clone());
            if !self.chain.contains_key(&key){
                self.chain.insert(key.clone(), vec![]);
            }
            self.chain[&key].push(vector[i+3].clone());
        }
    }
}
