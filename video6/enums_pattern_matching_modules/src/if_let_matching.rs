// using if let : match to a single pattern and ignore the rest

fn main(){
    only_three(Some(3));
    only_three(Some(6));
    only_three(None);
}
 
fn only_three(x: Option<i32>){
    if let Some(3) = x {
        println!("Matched 3!");
    }
}

// fn only_three(x: Option<i32>){
//     match x {
//         Some(3) => println!("Matched 3!"),              //println!() returns a unit type ()
//         _ => (),
//     }
// }