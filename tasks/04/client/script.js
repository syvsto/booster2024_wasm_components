const inputClusterCount = document.getElementById("input-cluster-count");
const btn = document.getElementById("btn-new-data");
btn.addEventListener("click", async () => {
  const points = genPoints();
  const unclustered = points.map((p) => ({ ...p, cluster: 1 }));
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

const genPoints = () => {
  const size = 50 + Math.floor(Math.random() * 20);
  const points = Array(size)
    .fill(0)
    .map((_) => ({
      x: Math.random() * 10,
      y: Math.random() * 10,
    }));
  return points;
};

const cluster = async (points) => {
  try {
    const clusterCount =
      inputClusterCount.value !== "" ? inputClusterCount.value : 2;
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
  } catch (error) {
    console.error(error);
  }
};

const zipObjects = (arr1, arr2) => arr1.map((k, i) => ({ ...k, ...arr2[i] }));
