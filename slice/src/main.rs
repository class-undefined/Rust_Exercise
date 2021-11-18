fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
}
fn get_words(_s: &String) -> Vec<String> {
        let mut word: String = String::new();
        let mut ans: Vec<String> = Vec::new();
        for (i, ch) in _s.as_str().chars().enumerate() {
            if ch != ' ' {
                word.push(ch);
            }
            if ch == ' ' || i == _s.len() - 1 {
                ans.push(word.clone());
                word.clear();
            }
        }
        ans
}

fn main() {

    test();
    /* 分割单词 */
    let s = String::from("hello world");
    let result: Vec<String> = get_words(&s);
    for str in result.iter() {
        println!("{}", str);
    }
    println!("{}", first_word(&s));
}
