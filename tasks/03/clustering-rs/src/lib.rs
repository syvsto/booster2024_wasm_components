use linfa::traits::{Fit, Predict};
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use ndarray;
use wasm_bindgen::prelude::wasm_bindgen;
use wit_bindgen;

wit_bindgen::generate!({
    world: "clustering",

    exports: {
        world: Clustering
    }
});

struct Clustering;
impl Guest for Clustering {
    fn categorize(points: Vec<Vec<f64>>, n_clusters: u8) -> Vec<u8> {
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
        let result: Vec<u8> = labels.iter().map(|label| label.clone() as u8).collect();

        result
    }
}

#[wasm_bindgen]
pub fn cluster(points_x: Vec<f64>, points_y: Vec<f64>, n_clusters: u8) -> Vec<u8> {
    let points: Vec<Vec<f64>> = points_x.iter().zip(points_y.iter()).map(|(x, y)| vec![*x, *y]).collect();
    Clustering::categorize(points, n_clusters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data_x = vec![-1., 1.0, 1.0, 15.3];
        let data_y = vec![-3., 1.1, 1.4, 16.2];
        let data = data_x.iter().zip(data_y.iter()).map(|(x, y)| vec![*x, *y]).collect();
        let res: Vec<u8> = Clustering::categorize(data, 3u8);
        println!("{:?}", res);
    }
}
