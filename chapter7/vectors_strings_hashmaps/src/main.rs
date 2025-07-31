use std::collections::HashMap;

// hash maps : HashMap<K, V>, they also store the data in the heap 
// if we want to store values by some meaningful keys and not indices

//creating a new HASH MAP
fn main() {
    let mut scores0 = HashMap::new();
    scores0.insert(String::from("Blue"), 10);        //keys of type String and values of type i32
    scores0.insert(String::from("Yellow"), 9);

    // using iterators to create a hash map or collect method on a vector of tuples

    let teams = vec![String::from("Black"), String::from("Pink")];
    let initial_scores = vec![11, 12];
    let mut scores1: HashMap<_, _> =            //HashMap<_, _> : this type annotation is used because we can collect other into many other data structures 
        teams.into_iter().zip(initial_scores.into_iter()).collect();        // the key is Strings and the values are i32

    
    //Hash maps and ownerships
    // for the types that implement the Copy traits, like i32 values are copied into the hash map
    // for owned values like String, the values will be moved and hasp map will be the owner of those values

    let field_name = String::from("Favourite Color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);        //we can not use the field_name and field_value after this, as they have been moved into the hash map with the call to insert

    // if we insert references into Hashmaps the values won't be moved into the hash maps. The values that the references point to must be valid for at least as long as the hash map is valid


    // accessing values in a hash map
    //1 way
    for (key, value) in &scores0{
        println!("{} : {}", key, value);
    }


    //2 way
    // Option<&V> -> reference to the value V, get does not return the value itself rather a reference to the value which is Some(&V)
    let team_name = String::from("Blue");
    let score = scores0.get(&team_name);

    //Updating a Hash Map
    //Overwriting a value
    //if key is already in the Hashmap and still we insert the data with the same key the original value will be overwritten
    scores0.insert(String::from("Blue"), 20);   // the value at key Blue is replaced with 20
    
    // debug format already present in the hashmap
    println!("{scores0:?}");

    //Only inserting a value when the key has no value
    // entry api to check if the value is present for a key, returns an enum "Entry"

    scores0.entry(String::from("Black")).or_insert(99);     // not present the new pair is added
    scores0.entry(String::from("Blue")).or_insert(100);     // blue was already present so the value does not update

    println!("{scores0:?}");
    

    //updating the value based on the old value
    // Iteration over a hashmap occurs in arbitrary order
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);       //returns a mutable reference to the value for the specified key (word)
        *count+=1;
    }

    println!("{map:?}")
}

