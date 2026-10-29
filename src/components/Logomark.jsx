import { memo } from "react";

/// The KlugNetz mark: the letter K drawn as a node-link graph.
///
/// The stem and the two arms are the edges, the five beaded terminals are the
/// nodes, and the vertex of the K is the junction where all three meet -- a
/// letterform and an adjacency list at the same time. It is one colour by
/// construction: the old mark was a purple-to-teal gradient, the only colour
/// in an otherwise monochrome interface, and it could not follow the theme.
///
/// The weights were settled by rendering: push the nodes any larger and the
/// letterform dissolves into a generic share glyph, shrink them and it is a
/// plain typographic K. At 8/6/4.5 the K is unmistakable at 32px and the beads
/// still read as nodes at 512.
///
/// Inlined as a component rather than loaded through `<img src>` so it paints
/// in `currentColor` and follows the light/dark theme for free. The discs are
/// drawn after the edges in the same colour, so the overlap leaves no seam.
function Logomark({ size = 16, className, title }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 48 48"
      fill="none"
      className={className}
      role={title ? "img" : undefined}
      aria-hidden={title ? undefined : "true"}
      aria-label={title}
    >
      {title && <title>{title}</title>}
      <g stroke="currentColor" strokeLinecap="round">
        {/* Stem, then the two arms out of the vertex. */}
        <path d="M13 10V38" strokeWidth="8" />
        <path d="M13 24L35 10" strokeWidth="6" />
        <path d="M13 24L35 38" strokeWidth="6" />
      </g>
      <g fill="currentColor">
        <circle cx="13" cy="10" r="4.5" />
        <circle cx="13" cy="24" r="4.5" />
        <circle cx="13" cy="38" r="4.5" />
        <circle cx="35" cy="10" r="4.5" />
        <circle cx="35" cy="38" r="4.5" />
      </g>
    </svg>
  );
}

export default memo(Logomark);
