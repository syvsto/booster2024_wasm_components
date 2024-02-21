use crate::exports::clustering::rs::cluster::Guest;
use linfa::traits::{Fit, Predict};
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use ndarray;
use wasm_bindgen::prelude::wasm_bindgen;
use wit_bindgen;

wit_bindgen::generate!({
    world: "clustering",

    exports: {
        "clustering:rs/cluster": Clustering
    }
});

struct Clustering;
impl Guest for Clustering {
    fn run(points: Vec<Vec<f64>>, n_clusters: u32) -> Vec<u32> {
        let array2: Vec<[f64; 2]> = points
            .iter()
            .map(|p| [p[0].clone(), p[1].clone()])
            .collect();

        let points_dataset = DatasetBase::from(ndarray::Array2::from(array2));

        let model = KMeans::params(n_clusters as usize)
            .tolerance(1e-3)
            .fit(&points_dataset)
            .expect("KMeans fitted");

        let labels = model.predict(&points_dataset);
        let result: Vec<u32> = labels.iter().map(|label| label.clone() as u32).collect();

        result
    }
}
