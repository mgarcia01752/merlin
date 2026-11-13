# Windows installer artwork

`merlin.ico` contains transparent 16–256 px variants of the existing MERLIN icon.
WiX3 uses `dialog.bmp` (493×312) and `banner.bmp` (493×58). Text areas remain
white for native installer readability; the left panel uses MERLIN navy/teal.
The executable, Start menu shortcut and Installed apps share this icon.

Reproduce with `python3 tools/branding-windows.py` (Pillow, development only).
`provenance.json` records input/output hashes. No original art is overwritten.

`installer-ui.wxs` adapts WiX3.14.1's InstallDir wizard to insert the shortcut
choice after path validation. Its upstream copyright, MS-RL text and source
provenance are retained in the adjacent wix-ui files and bundled notices.
