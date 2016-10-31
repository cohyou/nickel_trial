function drawCircle(ctx, x, y, size) {
    ctx.beginPath();
    ctx.arc(x, y, size, 0, Math.PI*2, false);
    ctx.stroke();
}

function draw() {
    // document.write("<p>JavaScript DOMContentLoaded テスト2</p>");
    var cv = document.getElementById('cv');
    var context = cv.getContext('2d');
    // context.rotate(45 * Math.PI / 180);
    context.fillRect(300, -300, 100, 100);
    context.fillRect(500, -300, 100, 100);
    context.fillRect(300, -100, 100, 100);
    context.fillRect(500, -100, 100, 100);

    drawCircle(context, 100, 100, 40);
    drawCircle(context, 200, 100, 40);
    drawCircle(context, 300, 100, 40);
}

document.addEventListener('DOMContentLoaded', function() {
    draw();
});
