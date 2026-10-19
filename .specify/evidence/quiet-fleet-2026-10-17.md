# Quiet fleet navigation — 2026-10-17

> Historical validation record. Results apply only to the revision and environment recorded below; they do not describe current release readiness.


Quiet opens with a responsive robot fleet. Selecting a system opens only its task
list; selecting work opens existing evidence/actions. Back returns to the fleet.
Switching to Quiet clears the system/query selection. Original robot portraits,
colors, motion settings, source freshness and permission checks are preserved.
Provider-only observations receive a system portrait derived from the provider
and current work rather than requiring an actor record.

Validated: generated renderer checks cover initial portraits, no initial task
list, provider-only data, 320px layout, attention/stale marks, system selection and
Back. Desktop checks include 36 evaluator scenarios and 54 portrait comparisons.
The native Qt 420×320 expanded-Quiet fixture shows three system robots and clicks
through to the task list and evidence. The collapsed-tile fixture remains separate.

Native dashboard desired size is 420×320 for the Quiet fleet and bounded at
960×720 for detailed/Orbit views. The compositor retains actual sizing authority.
Installed Omarchy window movement/resizing and focus acceptance remain host checks.
