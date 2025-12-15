use util::*;

fn main() {
    print("Ingresa el número límite: ");
    let limite: u64 = read_u64();
    let mut count = 0;
    let mut reglon = 1;

    for i in 1..=limite {
        print!("{:>5} ", i);
        count += 1;
        if count == reglon {
            println!();
            count = 0;
            reglon += 1;
        }
    }
}
