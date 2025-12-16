use util::*;

fn main() {
    print("Dime tu salario: ");
    let salario: f32 = read_f32();
    let mut impuesto: u32 = 0;

    println!(
        "|{:^14}|{:^9}%|{:^13}|",
        "Salario bruto", "Impuesto", "Salario neto"
    );
    println!("| ------------ | -------- | ----------- |");

    if salario <= 2000.0 {
        println!(
            "|{:>13} |{:>8}% |{:>12} |",
            salario,
            impuesto,
            salario - (salario * impuesto as f32 / 100.0)
        );
    } else if salario <= 5_000.0 {
        impuesto = 2;
        println!(
            "|{:>13} |{:8}% |{:>12} |",
            salario,
            impuesto,
            salario - (salario * impuesto as f32 / 100.0)
        );
    } else {
        impuesto = 5;
        println!(
            "|{:>13} |{:>8}% |{:>12} |",
            salario,
            impuesto,
            salario - (salario * impuesto as f32 / 100.0) - 500.0
        );
    }
    println!("| ------------ | -------- | ----------- |");
}
