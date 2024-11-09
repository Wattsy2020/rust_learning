fn main() {
    let cat = (String::from("Furry McFurson"), 3.5);
    let (ref name, age) = cat; // ref keyword binds a non reference as a reference
    println!("{:?}", cat);
    println!("{name} is {age} years old");
}
