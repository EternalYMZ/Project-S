// Run with Node.js. All inputs are permutations of ranks 0,...,n-1.
import assert from 'node:assert/strict';

function* permutations(values, k = 0) {
  if (k === values.length) {
    yield values;
    return;
  }
  for (let j = k; j < values.length; j++) {
    [values[k], values[j]] = [values[j], values[k]];
    yield* permutations(values, k + 1);
    [values[k], values[j]] = [values[j], values[k]];
  }
}

function gapPass(input, h) {
  const array = input.slice();
  let shifts = 0;
  for (let i = h; i < array.length; i++) {
    const value = array[i];
    let j = i;
    while (j >= h && value < array[j-h]) {
      array[j] = array[j-h];
      shifts++;
      j -= h;
    }
    array[j] = value;
  }
  return {array, shifts};
}

function gcd(a, b) {
  while (b) [a,b] = [b,a%b];
  return a;
}

const count = {threePassInputs: 0, tablesAndInputs: 0, sameClass: 0,
  crossClass: 0, onePassInputs: 0, gapsAndInputs: 0};

function checkThirdPass(original, afterSecond, h, g) {
  const d = gcd(h,g), H = h/d, G = g/d;
  let incidents = 0;
  for (let i = 0; i < original.length; i++) {
    const pivot = afterSecond[i], chain = i % g;
    const cls = chain % d, j = (chain-cls)/d;
    const lengths = Array(h).fill(0);
    const prefixes = Array(g).fill(0);
    const actual = Array(g).fill(0);
    for (let k = 0; k < original.length; k++) {
      if (original[k] < pivot) lengths[k%h]++;
      if (afterSecond[k] < pivot) prefixes[k%g]++;
      if ((k<i && afterSecond[k]>pivot) || (k>i && afterSecond[k]<pivot)) actual[k%g]++;
    }
    assert.equal(actual[chain], 0);
    // Reconstruct endpoints from the original source chains, then extend the arc one phase at a time.
    const endpoints = Array(G).fill(0);
    for (let s = 0; s < H; s++) endpoints[(s + H*lengths[cls+d*s]) % G]++;
    let Q = 0;
    for (let width = 1; width < G; width++) {
      const target = (j+H*width) % G;
      Q += endpoints[target];
      assert.equal(Math.abs(Q-Math.floor(H*width/G)), actual[cls+d*target]);
      count.sameClass++;
    }
    for (let target = 0; target < g; target++) {
      if (target % d !== cls) {
        const before = prefixes[chain] + Number(target < chain);
        assert.equal(Math.abs(prefixes[target]-before), actual[target]);
        count.crossClass++;
      }
      incidents += actual[target];
    }
  }
  assert.equal(incidents, 2*gapPass(afterSecond,1).shifts);
  count.tablesAndInputs++;
}

function checkOnePassEnergy(input, h) {
  const n = input.length;
  let packetEnergy = 0, correction = 0;
  for (let threshold = 1; threshold <= n; threshold++) {
    const discrepancy = Array(h).fill(0);
    for (let i = 0; i < n; i++) {
      discrepancy[i%h] += Number(input[i]<threshold)-Number(i<threshold);
    }
    for (const e of discrepancy) packetEnergy += h*e*e;
  }
  for (let i = 0; i < n; i++) {
    const v = ((i-input[i])%h + h)%h;
    correction += v*(h-v);
  }
  const output = gapPass(input,h).array;
  const energy = output.reduce((sum, x, i) => sum+(i-x)**2, 0);
  assert.equal(energy, packetEnergy-correction);
  count.gapsAndInputs++;
}

for (let n = 1; n <= 8; n++) {
  for (const input of permutations(Array.from({length:n}, (_,i) => i))) {
    if (n <= 7) {
      for (let h = 1; h <= n+2; h++) checkOnePassEnergy(input,h);
      count.onePassInputs++;
    }
    if (n >= 4) {
      for (let h = 3; h < n; h++) {
        const afterFirst = gapPass(input,h).array;
        for (let g = 2; g < h; g++) checkThirdPass(input,gapPass(afterFirst,g).array,h,g);
      }
      count.threePassInputs++;
    }
  }
}
assert.deepEqual(count, {threePassInputs:46224, tablesAndInputs:659904, sameClass:9888936,
  crossClass:2151960, onePassInputs:5913, gapsAndInputs:52145});
console.log(JSON.stringify(count, null, 2));
