# Embedded surface assets

All downloaded source maps are CC0, https://polyhaven.com/license.

- Asphalt 03: https://polyhaven.com/a/asphalt_03 — Charlotte Baglioni (photography), Dario Barresi (processing).
- Concrete Floor 02: https://polyhaven.com/a/concrete_floor_02 — Rob Tuytel.
- Industrial Sunset 02 (Pure Sky): https://polyhaven.com/a/industrial_sunset_02_puresky — Jarod Guest (sky edits), Sergej Majboroda (original).

Run `python3 scripts/prepare-materials.py` with Pillow installed to download the 2K source maps and normalize PNGs to 8-bit RGB. `manifest.json` records source and output hashes. The GL normal and AO/roughness/metal maps are linear; albedo is sRGB. The HDR panorama remains linear and is resampled to an RGBA16Float cubemap by the native model library at startup. Runtime assets are embedded; there are no runtime network requests.
