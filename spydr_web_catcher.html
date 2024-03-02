<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>SPYDR Web Catcher</title>
  <style>
    body {
      margin: 0;
      padding: 0;
      overflow: hidden;
      background-color: #1a1a1a; /* Dark grey background color */
    }
    
    canvas {
      display: block;
      width: 100%;
      height: 100%;
    }
  </style>
</head>
<body>
  <canvas id="gameCanvas"></canvas>

  <script>
    const canvas = document.getElementById('gameCanvas');
    const ctx = canvas.getContext('2d');
    const screenWidth = window.innerWidth; // Adjusted to fit the screen width
    const screenHeight = window.innerHeight; // Adjusted to fit the screen height
    canvas.width = screenWidth;
    canvas.height = screenHeight;

    const spiderSize = 50; // Adjusted for better visibility on mobile screens
    const antSize = 40; // Adjusted for better visibility on mobile screens
    const webSymbol = "🕸️";

    const spiderSymbol = "🕸️";
    const antSymbol = "🐜";
    let spiderPos = { x: screenWidth / 2, y: screenHeight - 75 }; // Adjusted for better placement on mobile screens
    let ants = [];
    for (let i = 0; i < 10; i++) {
      let ant = {
        x: Math.random() * (screenWidth - antSize),
        y: Math.random() * (screenHeight - antSize),
        frozen: false
      };
      ants.push(ant);
    }

    let score = 0;

    function drawSpider() {
      ctx.fillStyle = 'white';
      ctx.font = spiderSize + 'px serif';
      ctx.fillText(spiderSymbol, spiderPos.x, spiderPos.y);
    }

    function drawAnts() {
      ctx.fillStyle = 'white';
      ctx.font = antSize + 'px serif';
      ants.forEach(ant => {
        if (!ant.frozen) {
          ctx.fillText(antSymbol, ant.x, ant.y);
        }
      });
    }

    function drawScore() {
      ctx.fillStyle = 'white';
      ctx.font = '30px serif';
      ctx.fillText('Score: ' + score, 10, 30); // Adjusted position for better visibility on mobile screens
    }

    function clearScreen() {
      ctx.fillStyle = '#1a1a1a'; // Dark grey background color
      ctx.fillRect(0, 0, screenWidth, screenHeight);
    }

    function checkCollision(x, y) {
      for (let i = 0; i < ants.length; i++) {
        let ant = ants[i];
        if (!ant.frozen && x >= ant.x && x <= ant.x + antSize && y >= ant.y && y <= ant.y + antSize) {
          ant.frozen = true;
          score++;
          if (score === 5) {
            window.alert("Congratulations! You won 1 SPYDR token!");
            // Add code to handle winning SPYDR token
          }
          break;
        }
      }
    }

    canvas.addEventListener('click', function(event) {
      let x = event.clientX;
      let y = event.clientY;
      checkCollision(x, y);
    });

    function moveAnts() {
      ants.forEach(ant => {
        if (!ant.frozen) {
          ant.x += Math.random() * 3 - 1;
          ant.y += Math.random() * 3 - 1;
        }
      });
    }

    function drawSpiderWebs() {
      ctx.fillStyle = 'white';
      ctx.font = antSize + 'px serif';
      ants.forEach(ant => {
        if (ant.frozen) {
          // Adjust the position to draw the spider web emoji slightly below the ant
          ctx.fillText(webSymbol, ant.x, ant.y + antSize);
        }
      });
    }

    function gameLoop() {
      clearScreen();
      drawSpider();
      drawAnts();
      drawSpiderWebs(); // Draw spider webs after drawing ants
      drawScore();
      moveAnts();
      requestAnimationFrame(gameLoop);
    }

    gameLoop();
  </script>
</body>
</html>
