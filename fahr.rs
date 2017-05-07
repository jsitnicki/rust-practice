/* Print Fahrenheit-Celsius table */
fn main() {
    let mut fahr: f32;
    let mut cels: f32;

    let lower: f32 = 0.0;
    let upper: f32 = 120.0;
    let step:  f32 = 5.0;

    println!("{:>3}\t{:>6}", "°F", "°C");
    println!("{:->3}\t{:->6}", "", "");

    fahr = lower;
    while fahr <= upper {
        cels = (5.0/9.0) * (fahr-32.0);
        println!("{:3.0}\t{:6.1}", fahr, cels);
        fahr = fahr + step;
    }
}
