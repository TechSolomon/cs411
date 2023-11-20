// benchmark.js
// Solomon Himelbloom
// 2023-11-17

// For CS 411 Fall 2023

let title = "📊 Convex Hull";

console.log(title + "\n");

input = [10, 100, 1000, 10000];

/*
(0,1) ----------------- (1,1)
    |                       |
    |                       |
    |         Unit          |
    |        Square         |
    |                       |
    |                       |
(0,0) ----------------- (1,0)
*/

function checkBoundary(points) {
  for (let i = 0; i < points.length; i++) {
    if (
      points[i].x < 0 ||
      points[i].x > 1 ||
      points[i].y < 0 ||
      points[i].y > 1
    ) {
      return false;
    }
  }
  return true;
}

function generateUnitSquare(n) {
    let points = [];
    for (let i = 0; i < n; i++) {
        points.push({
            x: Math.random(),
            y: Math.random(),
        });
    }
    return points;
}

for (let i = 0; i < input.length; i++) {
    console.log("Input: ", input[i]);
    console.log("Output: ", generateUnitSquare(input[i]));
    console.log("Boundary: ", checkBoundary(generateUnitSquare(input[i])));
    console.log();
}
