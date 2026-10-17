# Compact native Orbit layout — 2026-10-17

The 720px recording previously fell below a hard-coded 760dp two-column
breakpoint, forcing every system into a full-width row. That was authored Miilo
layout policy, not an Omarchy requirement.

The shared OWL view now computes one to three columns from actual available
width, with a 268dp minimum tile budget plus spacing. Tile widths are capped at
330dp; one system no longer expands across a desktop. Padding is 10dp and the
animation field is 164dp instead of 192dp. The original robot and projected
orbits use the same 0.85 scale, preserving the open nucleus and task tracks.
Task rows retain their 44dp interaction height.

The recording is still 720x680, 20.083 seconds and Tokyo Night. It now shows two
robots side by side. Native Qt captures also passed at 360px (one column) and
960px (three columns), including actual work-row selection. Generated renderer
tests assert the grid at six available widths and cap a single desktop tile.
The 36 desktop scenarios, motion tests, 54 exact portrait comparisons and
official OWL syntax checks all pass. The plugin portable checks pass with the
updated explicit provenance patch and asset hashes. These captures remain
isolated synthetic Qt fixtures, not real Omarchy shell acceptance.

The official Omarchy guide uses fitted content and inherited shell styles; it
does not prescribe Miilo's former breakpoint. Reference:
https://plugins.omarchy.org/develop.html

Community reference: the author of Responsive Dark Analytics Dashboard describes
container-query grids and reduced-motion handling in their own implementation:
https://dev.to/zulalnb/responsive-dark-analytics-dashboard-54n2

No release or deployment performed.
