# mandelbrot-wasm

Mandelbrot tree implementation with rust to compile for webasm

## Build

```bash
wasm-pack build --target web
```

## Usage

```js
  import init, { mandelbrot_pixels } from "./mandelbrot.js";

  await init();

  const canvas = document.getElementById("canvas");
  const width = parseInt(canvas.getAttribute("width"));
  const height = parseInt(canvas.getAttribute("height"));
  let ctx = canvas.getContext("2d");

  let pixels = Uint8ClampedArray.from(mandelbrot_pixels(width, height, 1000, -2.0, 1.0, -1.0, 1.0));
  let imageData = new ImageData(pixels, width, height);
  ctx.putImageData(imageData, 0, 0);
```