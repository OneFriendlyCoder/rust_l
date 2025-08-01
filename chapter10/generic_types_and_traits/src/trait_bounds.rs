use std::fmt::Display;


struct Pair<T>{
    x: T,
    y: T,
}


impl<T> Pair<T>{
    fn new(x: T, y T)-> Self{           //"Self": is used to refer to the implementing type inside an impl block
        Self{x,y}
    }
}

// conditionally implementing methods on a generic type depending on trait bounds
impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){          // point to an instance of the type
        if self.x >= self.y{
            println!("The largest member is x = {}", self.x);
        } else{
            println!("The largest number is y = {}", self.y);
        }
    }
}
