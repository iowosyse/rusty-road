use util::*;

fn main() {
    print("Cuantos numeros de fibonacci quieres ver? ");
    let limit = read_usize();
    let mut n0 = 0;
    let mut n1 = 1;

    for _ in 0..limit {
        print!("{} ", n0);
        let n2 = n0 + n1;
        n0 = n1;
        n1 = n2;
    }
}
