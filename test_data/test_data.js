function generateKochSnowflake(iterations, length) {
    const points = [];
    const startX = 400;
    const startY = 300;
    const angle = Math.PI / 3; // 60 degrees

    function addPoint(x, y) {
        points.push([x, y]);
    }

    function koch(x1, y1, x2, y2, iter) {
        if (iter <= 0) {
            addPoint(x1, y1);
            addPoint(x2, y2);
        } else {
            const dx = (x2 - x1) / 3;
            const dy = (y2 - y1) / 3;
            const x3 = x1 + dx;
            const y3 = y1 + dy;
            const x4 = x3 + dx * Math.cos(angle) - dy * Math.sin(angle);
            const y4 = y3 + dx * Math.sin(angle) + dy * Math.cos(angle);
            const x5 = x1 + 2 * dx;
            const y5 = y1 + 2 * dy;

            koch(x1, y1, x3, y3, iter - 1);
            koch(x3, y3, x4, y4, iter - 1);
            koch(x4, y4, x5, y5, iter - 1);
            koch(x5, y5, x2, y2, iter - 1);
        }
    }

    const height = Math.sin(angle) * length;
    const p1 = [startX, startY - height / 2];
    const p2 = [startX + length / 2, startY + height / 2];
    const p3 = [startX - length / 2, startY + height / 2];

    koch(p1[0], p1[1], p2[0], p2[1], iterations);
    koch(p2[0], p2[1], p3[0], p3[1], iterations);
    koch(p3[0], p3[1], p1[0], p1[1], iterations);

    return points;
}

function generateSerpentineCurve(length, waves, amplitude) {
    const points = [];
    const startX = 0;
    const startY = 300;

    for (let i = 0; i <= length; i++) {
        const x = startX + i;
        const y = startY + amplitude * Math.sin((i / length) * waves * 2 * Math.PI);
        points.push([x, y]);
    }

    return points;
}

export { generateKochSnowflake, generateSerpentineCurve }