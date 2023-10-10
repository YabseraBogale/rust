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
    //let count=LetterCounter{word:String::from("Hello World"),letter:'_',counter:0};
    let word=String::from("Hello World");
    let mut ccount=0;
    for i in word.chars(){
        if i=='l'{
            ccount+=1;
            println!("{}",i.to_string());
        }
    }
    println!("L in hello world is {}",ccount);
    
}
