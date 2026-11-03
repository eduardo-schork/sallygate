# App icon sources

The design system's icon files (v0.1). The PNG, ICO and ICNS files one folder up are generated from them:

- `sallygate-icon.svg`: Windows and Linux icons, `bun run tauri icon` input.
- `sallygate-icon-small.svg`: sizes up to 32 px (the door frame disappears below 32 px), including the 32 px PNG and the 16 and 32 px entries of `icon.icns`.
- `sallygate-icon-macos.svg`: `icon.icns` from 128 px, with the margin macOS expects around its mask.
- `sallygate-glyph.svg` and `sallygate-menubar-template.svg`: not used yet (a menu bar item later).

Never place the icon on a light background or recolor the light (design system, page 04).
