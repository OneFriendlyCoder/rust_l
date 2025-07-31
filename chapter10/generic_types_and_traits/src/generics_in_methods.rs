//generics in method definations
// implementing methods on enums and structs and using generics types on them as well
// struct Point<T>{
//     x: T,
//     y: T,
// }

// impl<T> Point<T>{           //the <T> in Point<T> needs to be introduced in that impl line
// // impl<U> Point<T>{           //but the above is conventional 
//     fn x(&self) -> &T{
//         &self.x
//     }
// }

// fn main(){
//     let p = Point{x: 5, y:10};
//     println!("p.x = {}", p.x());
// }


//A method on a generic struct can have its own generic parameters, separate from the structs parameters
struct Point<X1, Y1>{
    x: X1,
    y: Y1,
}


impl<X1, Y1> Point<X1, Y1>{
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2>{
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

fn main(){
    let p1 = Point{x: 5, y:10.4};
    let p2 = Point{x:"Hello", y:'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


//Monomorphization of generics : compiler takes the generic code and generates specific versions of it for each concrete type we use