// 呼び出す関数を draw_delaunay_pattern に変更
import init, { draw_delaunay_pattern } from './pkg/generator.js';

async function run() {
  await init();

  try {
    // <canvas>のIDと、生成するランダムな点の数を渡す
    // この数が多いほど、三角形は細かくなる
    draw_delaunay_pattern('my-canvas', 300); 
    console.log("Delaunay pattern drawn successfully!");
  } catch (error) {
    console.error("Error drawing pattern:", error);
  }
}

run();