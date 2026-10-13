# Attribution

Every asset bundled in this repository (images, icons, fonts, data files) is listed here with its
author, source and licence. `cargo xtask assets` fails if a file under `assets/`, `docs/brand/` or
`docs/images/` is missing from this list.

Nova Fig's user-interface icons are drawn in code (`crates/ui-egui/src/icons.rs`): original work,
MIT OR Apache-2.0. No third-party icon sets are used.

## Assets in this repository

| Path | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `assets/app-icon/` (all files: `novafig.svg`, `novafig-small.svg`, `novafig-*.png`, `novafig.ico`, `hicolor/`) | The project owner (engraving-style drawing, vectorised) | Original work, see [`assets/app-icon/README.md`](assets/app-icon/README.md); derived files rendered by `cargo xtask icon` | MIT OR Apache-2.0, [`assets/app-icon/LICENSE.txt`](assets/app-icon/LICENSE.txt) | Nova Fig app icon (chameleon) |
| `docs/brand/novafoods-logo.svg`, `docs/brand/novafoods-logo.png`, `docs/brand/novafoods-logo-white.svg`, `docs/brand/novafoods-logo-white.png`, `docs/brand/novafoods-mark.svg`, `docs/brand/novafoods-mark.png`, `docs/brand/novafoods-mark-black.svg`, `docs/brand/novafoods-mark-black.png` | Nova Foods Team | EuLucaDev/craftrules `assets/brand/` | Nova Foods brand terms (`docs/brand/LICENSE-brand.txt`), not open source | Trademarks; unmodified use within Nova Fig only |
| `docs/images/` (all screenshots and exports: `shot-*.png`, `art-*.png`) | Nova Fig contributors | Screenshots of Nova Fig showing synthetic demo documents made in Nova Fig, and PNG exports of those documents by Nova Fig's renderer | MIT OR Apache-2.0 | No third-party artwork |
| `examples/*.novafig` | Nova Fig contributors | Synthetic demo documents built through Nova Fig's command API; invented copy, shapes and vectors drawn from scratch, no images or embedded fonts | MIT OR Apache-2.0 | Text uses Inter (OFL, from novafoods-fonts or the system); no third-party artwork |

## Fonts (build input, not in this repository)

Release builds embed fonts from the optional novafoods-fonts build input
([EuLucaDev/novafoods-fonts](https://github.com/EuLucaDev/novafoods-fonts), `NOVAFOODS_FONTS_DIR`). Its own
`ATTRIBUTION.md` and licence files are authoritative; the packages ship each font's licence.

| Family | Licence | Upstream |
|---|---|---|
| Inter | SIL Open Font License 1.1 | https://github.com/rsms/inter |
| Roboto, Montserrat, Lato, Open Sans, Poppins, Nunito Sans, Merriweather, Playfair Display, Source Sans 3, Source Serif 4, Carlito, Caladea | SIL Open Font License 1.1 | Google Fonts / upstream repositories (see novafoods-fonts) |
| Liberation Sans, Serif, Mono | SIL Open Font License 1.1 | https://github.com/liberationfonts |
| BIZ UDPGothic, BIZ UDMincho, Shippori Mincho, Noto Sans Arabic | SIL Open Font License 1.1 | Google Fonts (see novafoods-fonts) |

egui's default fonts (compiled into the `egui`/`epaint` crates) keep their own licences (OFL / Apache-2.0 / Ubuntu Font Licence); see those crates.
