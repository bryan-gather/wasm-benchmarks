function getPerpendicularDistance(point, lineStart, lineEnd) {
    const [x, y] = point;
    const [x1, y1] = lineStart;
    const [x2, y2] = lineEnd;

    const area = Math.abs((x2 - x1) * (y1 - y) - (x1 - x) * (y2 - y1));
    const base = Math.hypot(x2 - x1, y2 - y1);
    return area / base;
}


function rdp(points, epsilon) {
    if (points.length < 3) {
        return points;
    }

    let dmax = 0;
    let index = 0;
    const end = points.length - 1;

    for (let i = 1; i < end; i++) {
        const d = getPerpendicularDistance(points[i], points[0], points[end]);
        if (d > dmax) {
            index = i;
            dmax = d;
        }
    }

    if (dmax > epsilon) {
        const recResults1 = rdp(points.slice(0, index + 1), epsilon);
        const recResults2 = rdp(points.slice(index), epsilon);

        return recResults1.slice(0, -1).concat(recResults2);
    } else {
        return [points[0], points[end]];
    }
}

export { rdp };