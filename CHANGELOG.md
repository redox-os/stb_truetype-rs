## master
* Add `is_collection(&[u8]) -> bool`.
* Remove most unsafe usages.
* `VertexType` implements `Eq`.

## 0.2.2
* Merge a number of bugfixes, update documentation links, add new debugging features.

## 0.2.1
* Fix `attempt to subtract with overflow` error in get_glyph_kern_advance.

## 0.2
* `FontInfo` is now generic in the storage for the font data, allowing flexible management of font data lifetimes. This is a breaking change.

## 0.1.2
* Fix for edge case behaviour for `get_glyph_pair_kern_advance` by switching to `i32` instead of `u32` to match stb_truetype.h (see issue #3).

## 0.1.1
* Fix for glyf table format 12 and 13 handling to match implementation in stb_truetype.h (see issue #2).

## 0.1
* Initial release.
