<html>
  <head>
  <title>A Simple Mustache Demo</title>
  <meta charset="utf-8">
  <script src='canvas.js'></script>
</head>
<body>
  <h1>A Simple Mustache Demo</h1>
  <form >
    <input type="text"></input>
    <button type="submit">push!</button>
  </form>
    <h4>Product Info: {{name}}</h4>
    <ul>
      <li>Product: {{name}}</li>
      <li>Color: {{color}}</li>
      <li>Price: ${{price}}</li>
    </ul>
    <canvas id="cv" width="800" height="800"></canvas>
</body>
</html>
