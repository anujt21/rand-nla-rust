// use ndarray::Array1;
// use rand::Rng;

// fn random_vector_dot(n: usize) -> f64 {
//     let mut rng = rand::thread_rng();
//     let v1 = Array1::from_iter((0..n).map(|_| rng.gen::<f64>()));
//     let v2 = Array1::from_iter((0..n).map(|_| rng.gen::<f64>()));
//     v1.dot(&v2)
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_dot() {
//         let res = random_vector_dot(5);
//         println!("Dot product: {}", res);
//         assert!(res.is_finite());
//     }
// }
