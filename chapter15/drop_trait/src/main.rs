// drop trait are destructors in C++
// Define cleanup logic when the value goes out of scope

// trait signature :    
trait Drop {            // these are like function signatures
    fn drop(&mut self);     // mutable reference to the self instance 
}

struct MyStruct {
    data: String,
}

impl Drop for MyStruct {                // gives the custom behaviour, like we define the destructor in C++, though the object would have been destroyed but then we can't have customized it
    fn drop(&mut self) {                // also provides the method body
        println!("Cleaning up: {}", self.data);
        self.data.clear(); // allowed because we have &mut self
    }
}

fn main() {
    let x = MyStruct{data: String::from("Hello") };
}
