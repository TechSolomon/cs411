import math
import time
import random
import matplotlib.pyplot as plt


def quick_hull(points):
    """Initial computation → 2D representation of points."""

    def recursive_hull(p, q, points, hull):
        """Recurse until no more points are left."""
        maximum_distance = 0
        extension_point = None

        for r in points:
            distance = abs(find_hull(p, q, r))
            if distance > maximum_distance:
                maximum_distance = distance
                extension_point = r

        if extension_point is not None:
            recursive_hull(
                p,
                extension_point,
                [
                    point
                    for point in points
                    if find_hull(p, extension_point, point) == 1
                ],
                hull,
            )
            hull.append(extension_point)
            recursive_hull(
                extension_point,
                q,
                [
                    point
                    for point in points
                    if find_hull(extension_point, q, point) >= 0
                ],
                hull,
            )

    def find_hull(sk, p, q):
        """Find the points that are outside the triangle."""
        value = (p[1] - sk[1]) * (q[0] - p[0]) - (p[0] - sk[0]) * (q[1] - p[1])
        if value == 0:
            return 0
        return 1 if value > 0 else -1

    if len(points) < 3:
        raise ValueError("[!] Minimum of 3 points required.")

    # Extremas (via x-coordinates)
    local_min, local_max = min(points), max(points)
    convex_hull = [local_min, local_max]

    recursive_hull(
        local_min,
        local_max,
        [point for point in points if find_hull(local_min, local_max, point) == 1],
        convex_hull,
    )
    recursive_hull(
        local_max,
        local_min,
        [point for point in points if find_hull(local_max, local_min, point) == 1],
        convex_hull,
    )

    return convex_hull


def graphical_output(input):
    """Generate a graph for each input that shows all the points
    and lines representing the convex hull."""
    points = [(random.uniform(0, 1), random.uniform(0, 1)) for _ in range(input)]
    convex_hull = quick_hull(points)
    print(f"{input} points = {time.time() - begin:.4f}s")

    X, Y = [p[0] for p in points], [p[1] for p in points]

    # Configuration
    plt.scatter(X, Y, s=5, color="black")
    plt.xlim(0, 1)
    plt.ylim(0, 1)
    plt.xlabel("X")
    plt.ylabel("Y")

    # Counter-Clockwise Order
    convex_hull.sort(
        key=lambda p: (
            math.atan2(p[1] - points[0][1], p[0] - points[0][0]) + 2 * math.pi,
            p[0],
        )
    )

    # System Outline
    hull_X = [p[0] for p in convex_hull]
    hull_Y = [p[1] for p in convex_hull]

    # Last Point + First Point
    hull_X.append(convex_hull[0][0])
    hull_Y.append(convex_hull[0][1])

    plt.plot(hull_X, hull_Y, color="red")
    plt.title("Quickhull (" + str(input) + " points)")
    return plt


if __name__ == "__main__":
    # 2D points randomly distributed inside the unit square.
    print(">> 🤖 Generating random points...")

    sample = [100]  # n = 10, 100, 1000, & 10000
    plots = []

    begin = time.time()
    for i in sample:
        plots.append(graphical_output(i))
    end = time.time()

    for plot in plots:
        plot.show()

    print(f">> ⏰ Total time elapsed: {end - begin:.4f}s")
