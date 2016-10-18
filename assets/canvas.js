function draw() {
    // document.write("<p>JavaScript DOMContentLoaded テスト2</p>");
    var cv = document.getElementById('cv');
    var context = cv.getContext('2d');
    context.rotate(45 * Math.PI / 180);
    context.fillRect(300, -300, 100, 100);
    context.fillRect(500, -300, 100, 100);
    context.fillRect(300, -100, 100, 100);
    context.fillRect(500, -100, 100, 100);

}

document.addEventListener('DOMContentLoaded', function() {
    draw();
});
