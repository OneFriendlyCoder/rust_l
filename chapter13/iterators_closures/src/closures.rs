//CLosures : anonymous functions that can be saved as variables and passed to other functions as arguments

//capturing the environment variables with closures
// Debug : allows printing with {:?}
// PartialEq : allows comparing enum values using ==
// Copy and Clone : allows the enum values to be duplicated for passing them around without ownership issues 

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor{
    Red, 
    Blue,
}

struct Inventory{
    shirts: Vec<ShirtColor>,            //Vec<ShirtColor> : a vector that stores the ShirtColor
}

impl Inventory{
    // &self : a reference to the current instance of the struct 
    // let store = Inventory {...};         //inside the giveaway method self refers to the store variable
    // store.giveaway(...);
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }


    fn most_stocked(&self) -> ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts{
            match color{
                ShirtColor::Red => num_red+=1,
                ShirtColor::Blue => num_blue+=1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        }else{
            ShirtColor::Blue
        }
    }
}

fn main(){
    let store = Inventory{
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2: Option<ShirtColor> = None;
    let giveaway2 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

}
