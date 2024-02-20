import * as wasm from "clustering-rs";
import init from "clustering-rs";

init();

const inputClusterCount = document.getElementById("input-cluster-count");
const btn = document.getElementById("btn-new-data");

btn.addEventListener("click", async (e) => {
  e.preventDefault();
  const points = genPoints();
  const clustered = await cluster(points);

  const vlSpec = {
    $schema: "https://vega.github.io/schema/vega-lite/v5.json",
    data: {
      values: clustered,
    },
    mark: "point",
    encoding: {
      x: {
        field: "x",
        type: "quantitative",
        axis: {
          title: "X",
        },
      },
      y: {
        field: "y",
        type: "quantitative",
        axis: {
          title: "Y",
        },
      },
      color: { field: "cluster", type: "nominal" },
    },
  };
  vegaEmbed("#app", vlSpec);
});

const cluster = async (points) => {
  try {
    const clusterCount =
      inputClusterCount.value !== "" ? Number(inputClusterCount.value) : 2;

    if (points.length < 60) {
      return clusterClientside(points, clusterCount);
    } else {
      return await clusterServerside(points, clusterCount);
    }
  } catch (error) {
    console.error(error);
  }
};

const clusterClientside = (points, clusterCount) => {
  console.log("Clustering with Rust");
  const xs = points.map((p) => p.x);
  const ys = points.map((p) => p.x);
  const cs = wasm.cluster(xs, ys, clusterCount);
  const clusterObjs = Array.from(cs).map((d) => ({ cluster: d }));
  return zipObjects(points, clusterObjs);
};

const clusterServerside = async (points, clusterCount) => {
  const ps = points.map((p) => [p.x, p.y]);
  const res = await fetch("/cluster", {
    method: "POST",
    headers: {
      "Content-Type": "text/plain",
    },
    body: JSON.stringify({ points: ps, n_clusters: Number(clusterCount) }),
  });
  const json = await res.json();
  const clusterObjs = json.map((d) => ({ cluster: d }));
  return zipObjects(points, clusterObjs);
};

/* ----------------- */
/* Utility functions */
/* ----------------- */
const genPoints = () => {
  const size = 100 + Math.floor(Math.random() * 20);
  const points = Array(size)
    .fill(0)
    .map((_) => ({
      x: Math.random() * 10,
      y: Math.random() * 10,
    }));
  return points;
};

const zipObjects = (arr1, arr2) => arr1.map((k, i) => ({ ...k, ...arr2[i] }));
