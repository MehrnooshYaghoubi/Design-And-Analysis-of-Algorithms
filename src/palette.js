// Pastel fills for the comparison bars.
//
// Two constraints pull against each other here. A colour has to belong to an
// *algorithm*, not to a row: the bars are ranked fastest-first, so a palette
// indexed by position recolours the whole chart the moment one sort overtakes
// another. But a colour also has to be *distinguishable* from its neighbours,
// and hashing a name straight onto the hue circle does not guarantee that --
// "bucket sort" and "heap sort" hash six degrees apart, which reads as one
// colour.
//
// So: a fixed ring of hues that are far apart by construction, a name hashed
// to a starting slot, and linear probing when two names want the same slot.
// Assignment runs over the names in alphabetical order, never in rank order,
// so the result is stable across runs and across re-sorts.

/** FNV-1a, 32-bit. Small, stable, and good enough to scatter short names. */
function hash(str) {
  let h = 0x811c9dc5;
  for (let i = 0; i < str.length; i++) {
    h ^= str.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  return h >>> 0;
}

/// Hues at roughly 25-degree spacing, skewed away from the yellow-green band
/// where a pastel turns muddy. Fourteen slots covers every family in the app
/// with room to spare; past that the ring wraps and hues begin to repeat.
const HUES = [
  8, 28, 48, 68, 96, 128, 152, 174, 196, 216, 238, 262, 290, 320,
];

/// Dark-theme pastels sit against a near-black inset, so they can stay light
/// and brighten on hover. On white the same lightness washes out, so the light
/// theme runs a touch deeper and darkens on hover instead.
const WEIGHT = {
  dark: { s: 58, l: 74, hoverL: 85 },
  light: { s: 56, l: 68, hoverL: 57 },
};

/** One hue rendered at the current theme's pastel weight. */
export function pastel(hue, theme = "dark", hovered = false) {
  const { s, l, hoverL } = WEIGHT[theme] ?? WEIGHT.dark;
  return `hsl(${hue} ${s}% ${hovered ? hoverL : l}%)`;
}

/**
 * Maps each name to a hue, keeping every hue in the set distinct while there
 * are free slots. Returns a `Map<string, number>`.
 *
 * Probing trades a little stability for separation: a name that collides can
 * land on a different slot depending on which other names are present, so a
 * hue is fixed for a given selection and across re-sorts of it, but not
 * forever across every selection. Two bars no one can tell apart is the worse
 * failure of the two.
 */
export function assignHues(names) {
  const taken = new Array(HUES.length).fill(null);
  const hues = new Map();

  // Alphabetical, so the assignment does not depend on the order the results
  // happened to arrive in.
  for (const name of [...new Set(names)].sort()) {
    const start = hash(name) % HUES.length;
    let slot = start;
    // Probe forward for a free slot; if every slot is taken, keep the hashed
    // one and accept the repeat.
    for (let i = 0; i < HUES.length; i++) {
      const candidate = (start + i) % HUES.length;
      if (taken[candidate] === null) {
        slot = candidate;
        break;
      }
    }
    taken[slot] = name;
    hues.set(name, HUES[slot]);
  }

  return hues;
}
