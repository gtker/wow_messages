# `FullMonsterMoveSpline`

A `u32` point count followed by one full `Vector3d` (`x`, `y`, and `z` as `f32`) for every spline point.

Wrath `MonsterMoveData` uses this layout when `spline_flags` contains `FLYING` or `CATMULLROM`. Linear paths use [`MonsterMoveSpline`](monster-move-spline.md), which packs intermediate points as signed offsets.
