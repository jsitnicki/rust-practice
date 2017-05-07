/* Print Celsius-Fahrenheit table */
fn main() {
    let mut cels: f64;
    let mut fahr: f64;

    let lower = -20.0;
    let upper =  50.0;
    let step  =   2.5;

    println!("{: >6}\t{: >3}", "°C", "°F"); // headings
    println!("{:->6}\t{:->3}", "--", "--"); // horizontal lines

    cels = lower;
    while cels <= upper {
        fahr = (9.0/5.0) * cels + 32.0;
        println!("{:6.1}\t{:3.0}", cels, fahr);
        cels += step;
    }
}
