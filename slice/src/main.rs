fn main() {
    /* 分割单词 */
    let s = String::from("hello world");
    fn get_word(_s:&String) -> Vec<String> {
        let mut word: String = String::new();
        let mut ans: Vec<String> = Vec::new();
        for (i, ch) in _s.as_str().chars().enumerate() {
            if ch != ' ' {
                word.push(ch);
            }
            if ch == ' ' || i == _s.len() - 1{
                ans.push(word.clone());
                word.clear();
            }
        }
        ans
    }
    let result:Vec<String> = get_word(&s);
    for str in result.iter() {
        println!("{}", str);
    }
}
