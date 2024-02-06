use linfa::traits::{Fit, Predict};
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use ndarray;
use wit_bindgen;

wit_bindgen::generate!({
    world: "clustering",

    exports: {
        world: Clustering
    }
});

struct Clustering;
impl Guest for Clustering {
    fn categorize(points: Vec<(f64, f64)>, n_clusters: u8) -> Vec<(u8, (f64, f64))> {
        let array2: Vec<[f64; 2]> = points
            .iter()
            .map(|(px, py)| [px.clone(), py.clone()])
            .collect();

        let points_dataset = DatasetBase::from(ndarray::Array2::from(array2));

        let model = KMeans::params(n_clusters as usize)
            .tolerance(1e-3)
            .fit(&points_dataset)
            .expect("KMeans fitted");

        let labels = model.predict(&points_dataset);
        let result: Vec<(u8, (f64, f64))> = labels
            .iter()
            .zip(points)
            .map(|(label, point)| (label.clone() as u8, point))
            .collect();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = vec![(-1., -3.), (1.0, 1.1), (1.0, 1.4), (15.3, 16.2)];
        let res = Clustering::categorize(data, 3u8);
        println!("{:?}", res);
    }
}
