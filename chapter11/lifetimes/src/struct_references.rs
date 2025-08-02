// Structs with references
'// 'a implicity connects i.part back to novel
struct SomethingImp<'a>{        //<'a> 
    part: &'a str,          //cz this a reference we will need to tell rust how long this reference is valid
}

fn main(){
    let novel = String::from("This is a test.");
    let first_sentence = novel.split(' ').next().unwrap();
    let i = SomethingImp{
        part: first_sentence,
    };

    println!("{}", i.part);
}


// why not static ? static is too restrictive
struct SomethingImp{
    part: &'static str,         // part must be a reference to data that lives for the entire program
}


// lifetime annotations in methods
// implementing lifetimes on struct methods
// the syntax is same as generic syntax for impl
// impl<T> IMportantExcerpt<T{}

struct ImportantExcerpt<'a>{
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {     // this line is a syntax same as when using generic types with struct     
    fn level(&self) -> i32 {     
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
