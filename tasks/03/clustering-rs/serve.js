const express = require("express");
const app = express();
const port = 3000;
const wasm = require("./pkg/clustering_rs");

app.get("/", (req, res) => {
  var points_x = new Float64Array([1, 2, 3]);
  var points_y = new Float64Array([3, 2, 1]);
  var n_clusters = 3;

  var result = wasm.cluster(points_x, points_y, n_clusters);
  res.send(result.toString());
});

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`);
});
