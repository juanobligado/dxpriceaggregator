

pub fn mean(list: &[f64]) -> f64{
    let total: f64 = Iterator::sum(list.iter());
    f64::from(total) / (list.len() as f64)
}