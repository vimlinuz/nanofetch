function intoGb(value) {
  return value / (1024 * 1024);
}

const totalMemory = 16157772;
const freeMemory = 12264516;
const usedMemory = totalMemory - freeMemory;

(() => {
  console.log(
    `Memory | ${intoGb(usedMemory).toFixed(2)}/ ${intoGb(totalMemory).toFixed(2)}`,
  );
})();
