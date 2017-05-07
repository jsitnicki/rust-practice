/* Print Fahrenheit-Celsius table */
fn main() {
    let mut fahr;
    let mut cels;
    let lower = 0;
    let upper = 120;
    let step  = 5;

    fahr = lower;
    while fahr <= upper {
        cels = 5 * (fahr-32) / 9;
        println!("{}\t{}", fahr, cels);
        fahr = fahr + step;
    }
}
