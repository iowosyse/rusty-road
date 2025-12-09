fn main() {
    let name: &str = "Cande";
    println!("Hi, this is {name} from Rust");

    //Print con argumentos posicionales
    let x: i32 = 2;
    println!("{0} * {0} = {1}", x, x * x);
}
