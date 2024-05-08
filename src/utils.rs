pub fn distance(a: [f64; 2], b: [f64; 2]) -> f64{
    return (((a[0] - b[0]).abs()).powf(2.) + ((a[1] - b[1]).abs()).powf(2.)).sqrt();
}
