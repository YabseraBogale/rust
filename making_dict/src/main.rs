#[derive(Debug)]
struct LetterCounter{
    word: String,
    letter: char,
    counter: i32,
}

impl LetterCounter {
    fn count_letter(&self,l:char)->i32{
        let mut counted=0;
        for i in self.word.chars(){
            if l==i{
                counted+=1;
            }
        }
        counted
    }
}

fn main() {
    let count=LetterCounter{word:String::from("Hello World"),letter:'_',counter:0};
    let word=count.word.clone();
    let counted=count.count_letter('l');
    println!("the character in {:?} is counted {}",word,counted);
}
