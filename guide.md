* {
  box-sizing: border-box;
}

html,
body {
  margin: 0;
  height: 100%;
  overflow: hidden;
}

main {
  width: 100%;
  height: 100%;
  background-color: #1d1e22;
}

div {
  background-color: #878787;
  width: 2px;
  height: 2px;
}

And the JS file
const starsWrapper = document.querySelector("main");
const stars = new Array(200).fill(0).map(() => {
  const star = document.createElement("div");
  starsWrapper.append(star);

  return {
    star,
    x: Math.random() * window.innerWidth,
    y: Math.random() * window.innerHeight,
    r: Math.random() * 2
  };
});

function update() {
  stars.forEach((star) => {
    star.x += star.r;
    if (star.x > window.innerWidth) {
      star.x = 0;
    }

    star.star.style.transform = `translate(${star.x}px, ${star.y}px) scale(${star.r})`;
    star.star.style.opacity = `${star.r / 2}`;
  });
  requestAnimationFrame(update);
}

requestAnimationFrame(update);