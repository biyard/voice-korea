import path from "path";

export const latency = Number(process.env.LATENCY) || 0;
export const longlatency = Number(process.env.LONGLATENCY) || 0;
export const email = "biyard.test.0@gmail.com";
export const password = "abcd1234!";
export const image_path = path.join(__dirname, "../fixtures/images/");
export const screenshot_path = (spec, scenario, name) =>
  path.join(__dirname, `../screenshots/${spec}-spec/${scenario}/${name}.png`);
