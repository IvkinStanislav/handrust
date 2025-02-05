pub mod divide_and_conquer;
pub mod dp;
pub mod greedy;
pub mod practice;
pub mod sequential;
pub mod strategy;

#[cfg(test)]
fn f64_round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}
