#[derive(Debug)]
struct LetterCounter{
    word: String,
    letter: char,
    counter: i32,
}

impl LetterCounter {
    fn count_letter(&self,l:char)->i32{
        let mut count=0;
        for i in self.word.as_bytes(){
            if l.to_string()==i.to_string(){

            }
        }
        count
    }
}

fn main() {
    let count=LetterCounter{word:String::from("Hello World"),letter:'_',counter:0};
    
    
}
