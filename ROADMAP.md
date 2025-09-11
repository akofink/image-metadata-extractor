# Roadmap: Image Metadata Extractor (Client‑Side Only)

This roadmap outlines a privacy‑first, fully client‑side evolution of the Image Metadata Extractor. It is organized into phases with clear goals, user‑facing features, technical notes, and testing criteria. Dates are intentionally omitted; sequence may adapt based on community needs and browser platform capabilities.

Guiding principles:
- 100% client‑side (no servers, no tracking, no external storage of user data).
- Privacy by design (opt‑in UI features, safe defaults, no network calls required for core features).
- Rust + WebAssembly + Yew (type safety, performance, maintainability).
- Performance and accessibility are first‑class requirements.

Non‑goals:
- No server‑side processing or persistent cloud storage.
- No analytics/telemetry that sends data off device.
- No reliance on paid third‑party APIs for core functionality.

---

## What will make this the most useful tool on the Internet

Unique differentiators (all client‑side):
- Archive & dataset native: Import ZIP/7z/TAR archives, deduplicate by hash, and process at scale without uploading files anywhere.
- Cross‑file intelligence: Timeline view, location clustering heatmap, and cross‑image diff/dedupe to analyze entire shoots.
- Privacy score & guidance: Automatic sensitive‑field warnings, GPS precision fuzzing, and share‑safety checklist with clear remediation.
- Templated reporting: Custom, user‑defined report templates (Markdown/HTML/TXT) with local profile storage and schema versioning.
- Minimal video metadata: Parse container metadata (MP4/MOV) and embedded XMP/EXIF sidecar basics to extend usefulness beyond still images.
- Power‑user experience: Command palette, keyboard shortcuts, rule‑based redaction engine, and offline quick‑map preview (no external tiles).
- Internationalization: Built‑in multilingual UI and field explanations, shipped with the app (no network fetch).

---

## Phase 1 — Quality, UX, and Export Depth

Objectives: improve the core experience, expand export options, and enable multi‑file workflows while keeping memory use and performance tight.

User‑facing features:
- Batch support (multi file):
  - Drag‑and‑drop multiple images; process sequentially with progress reporting.
  - Export combined metadata (single JSON/CSV/TXT) and per‑file outputs.
  - Batch cleaning; optional ZIP download of cleaned files.
- Archive input:
  - Import ZIP/TAR (and explore 7z feasibility) directly; list, filter, and process selected files.
  - Streamed extraction in WASM to avoid large memory spikes.
- Export enhancements:
  - Additional formats: YAML, XML, and Markdown report.
  - Templated reports: user‑defined templates for Markdown/HTML/TXT with local persistence.
  - Per‑category export profiles (save/load locally) for repeatable selections.
  - Copy to clipboard for JSON/CSV/TXT/Markdown.
  - Stable schemas: include schema version, deterministic ordering, and optional JSON‑LD context.
- UX polish:
  - Dark mode and system theme auto‑detection.
  - Command palette and keyboard shortcuts for import, select all, export, clean.
  - Improved keyboard navigation and focus management.
  - Persistent UI preferences (theme, default export options) via localStorage.
  - Basic i18n: at least EN + one additional language, with local bundles.
- GPS privacy options:
  - Degrade precision (fuzz or round to N decimal places/meters) before export.
  - Geofencing presets (e.g., home/work polygons) to auto‑redact within defined areas.
  - Easy copy of map links (Apple/Google/OSM) without contacting third parties by default.
- Integrity checks & hashing:
  - SHA‑256 hashing for files (for dedupe and provenance reporting).
  - Quick consistency checks (e.g., GPSRef vs. sign, DateTime vs. SubSec, Orientation presence).

Technical notes:
- Use Web Workers to keep UI responsive for long‑running parsing/exports.
- Consider OffscreenCanvas for image cleaning in workers where supported.
- Client‑side ZIP creation for batch export/clean using a WASM‑friendly crate.
- Archive readers (ZIP/TAR) compiled to WASM; stream via Blob.slice and async iterators.
- Avoid large in‑memory spikes by streaming reads and chunked processing.

Testing & acceptance:
- wasm‑bindgen‑test coverage for batch flows, archive import, templates, and new export formats.
- Cross‑browser manual matrix for Chrome/Firefox/Safari/Edge.
- Performance target: process 50 JPEGs (24MP) or a 1–2 GB ZIP without tab lockup on a mid‑range laptop.

---

## Phase 2 — PWA, Persistence, and Advanced Cleaning Controls

Objectives: make the app installable and resilient offline, improve large‑dataset workflows, and provide granular cleaning/redaction controls.

User‑facing features:
- PWA: installable, offline support, app icon, splash screen.
- Session persistence:
  - IndexedDB for caching parsed metadata, thumbnails, and archive indices (opt‑in).
  - Resume interrupted batch operations.
- Directory import (where supported):
  - File System Access API to select folders, with recursive scanning.
- Advanced cleaning controls:
  - Redact fields by category or key pattern; custom rulesets.
  - Timestamp shift (e.g., +/‑ offset) for export without revealing exact times.
  - Preserve orientation/ICC selectively while stripping identifiers.
  - Write‑back (limited): allow author/copyright insertion and removal of selected fields for JPEG/PNG where safe.
- Side‑by‑side diff view: Original vs. Cleaned metadata comparison.
- Offline quick‑map preview:
  - Lightweight, internal SVG world map with projected coordinate dot for basic visualization (no external tiles).

Technical notes:
- Cross‑origin isolation (COOP/COEP) + threads for WASM SIMD/parallelism where possible.
- IndexedDB abstraction for safe, bounded storage (eviction strategy, quotas).
- Rule engine for cleaning: compile a set of predicates to include/exclude/transform keys.
- Minimal EXIF write path for limited, safe tag updates; robust validation to avoid corrupting files.

Testing & acceptance:
- Offline functionality verification (no network required for core use).
- Property‑based tests for rule engine transformations and write‑back safety.
- Bundle size target: keep release WASM under a reasonable limit; document tradeoffs.

---

## Phase 3 — Metadata Coverage, Analytics, and Forensics

Objectives: deepen metadata parsing breadth, cross‑file analytics, and non‑destructive inspection tools for photographers, journalists, and analysts.

User‑facing features:
- Metadata breadth:
  - IPTC and XMP parsing and export (where available in files).
  - MakerNotes surface (best‑effort, vendor‑specific keys with disclaimers).
- Cross‑file analytics:
  - Timeline view grouped by day/session; clock‑skew helper for multi‑camera shoots.
  - Location clustering and heatmap (SVG) from GPS points; quick anonymization toggles.
  - Multi‑image metadata diff (select N images and compare).
- Forensics/insight tools (client‑only):
  - Quick histogram and dominant colors visualization.
  - Detected color profile and bit depth reporting.
  - Basic thumbnail preview extraction from EXIF (if present).
  - Basic ELA (error level analysis) preview for JPEG to highlight recompression areas.
- Smart suggestions: flag potentially sensitive fields before sharing and compute a privacy score.

Technical notes:
- Evaluate existing Rust crates for IPTC/XMP (and adapt for WASM where needed).
- Consider a fall‑back lightweight JS parser for specific formats if unavoidable, behind a feature flag.
- All visualizations rendered locally (Canvas/SVG) with no external calls.
- ELA: re‑encode at a set quality in‑memory and diff; ensure performance safeguards.

Testing & acceptance:
- Regression suite of sample files across vendors and formats.
- Golden tests for parsed structures (EXIF/IPTC/XMP) and exports.
- Visual regression tests for charts/heatmaps (snapshot PNG or DOM assertions in wasm tests).

---

## Phase 4 — Formats, Platforms, and Packaging

Objectives: broaden file format compatibility and distribution options while maintaining the privacy promise.

User‑facing features:
- Additional formats (best‑effort, feature‑flagged by browser support):
  - HEIC/HEIF metadata extraction (where browser decoders or libraries allow).
  - RAW subsets (DNG focus) for metadata only; no full raw decode.
  - Minimal video metadata: container atoms (MP4/MOV), codec, duration, and embedded EXIF/XMP where present.
- Packaging:
  - Browser extension (WebExtension) variant that adds a context menu: “Inspect metadata” on images.
  - Bookmarklet / URL import (optional, with CORS caveats): paste URL to fetch and inspect; explicit user action, not required for core.
  - Standalone static build with service worker for easy offline distribution.

Technical notes:
- Use feature gating to keep the core bundle lean and only enable format‑specific code in targeted builds.
- WebCodecs API investigation for HEIF/AVIF support paths (capability detection at runtime).
- MP4 box parser in Rust/WASM for container metadata only (no decode).

Testing & acceptance:
- Capability matrix per browser; graceful fallbacks when unsupported.
- Size and performance budgets per packaging target.
- CORS handling documented for URL import; never required for core features.

---

## Cross‑Cutting Concerns

Accessibility (WCAG 2.1 AA):
- Semantic structure, focus order, skip links, ARIA where needed.
- High‑contrast theme and reduced motion option.
- Keyboard shortcuts for common actions (open, select all, export, clean).

Security & privacy:
- Strict CSP, COOP/COEP for threading, no eval.
- No network requests for core features; make optional external link opening explicit.
- Validate and sanitize file inputs; defensive parsing with safe Rust APIs.
- Local privacy: keep all caches opt‑in with clear controls to purge.

Performance:
- Web Workers by default for heavy tasks; OffscreenCanvas when available.
- Consider Rayon‑like threading in WASM (when cross‑origin isolation is enabled).
- SIMD for hot paths if it materially improves performance.
- Deterministic outputs and stable sort orders for reproducible exports.

Internationalization:
- Localized UI and field explanations.
- Locale switcher with persisted preference; no runtime network fetches.
- Right‑to‑left layout support where applicable.

Developer experience:
- Expand wasm‑bindgen tests covering UI logic in components.
- Add property‑based tests for parsing edge cases.
- Tooling tasks: bundle size check, flamegraph‑like profiling notes for WASM.
- JSON schema definitions for exports with versioning and compatibility tests.

---

## Milestones Overview

- M1 (Phase 1): Batch support, archive import, templated reports, dark mode, clipboard copy, GPS fuzz, hashing & integrity checks.  
  Done when: multi‑file/archives stable, templates ship, accessibility and theme toggles ship.

- M2 (Phase 2): PWA, IndexedDB session persistence, directory import, cleaning rule engine, limited write‑back, diff view, offline quick‑map.  
  Done when: installable offline app, resume flows, granular cleaning rules tested.

- M3 (Phase 3): IPTC/XMP parsing, cross‑file analytics (timeline/heatmap/diff), forensics (histogram/colors/ELA), suggestions & privacy score.  
  Done when: broader metadata reliably parsed/exported with visual insights.

- M4 (Phase 4): HEIC/HEIF/DNG exploration, minimal video metadata, WebExtension + bookmarklet packaging.  
  Done when: capability‑gated builds ship with graceful fallbacks.

---

## Implementation Hints (Codebase Mapping)

- UI components (`src/components/`):
  - New components: `batch_manager.rs`, `archive_import.rs`, `preferences.rs`, `diff_view.rs`, `rules_editor.rs`, `timeline.rs`, `map_preview.rs`, `command_palette.rs`.
  - Extend existing `metadata_export.rs` for new formats, templates, clipboard support, and schema versioning.
- Core logic:
  - `exif_core.rs` and `metadata_info.rs`: extend to include IPTC/XMP, rules engine, integrity checks, and suggestions/privacy score.
  - `binary_cleaner.rs`: support selective redaction, timestamp shift, and limited safe write‑back.
  - `export.rs`: templating engine, JSON‑LD/context, and schema stability.
  - `utils_wasm.rs`: web workers, OffscreenCanvas, clipboard, file streams, hashing.
  - New modules: `archive.rs` (ZIP/TAR), `video_meta.rs` (MP4/MOV container only), `i18n.rs` (local bundles), `map.rs` (SVG map projection/markers).
- Storage:
  - `utils_wasm.rs` + new `storage.rs` for localStorage/IndexedDB abstractions.
- Testing:
  - `tests/wasm/` additions for multi‑file, archive import, PWA offline mocks, rule engine, templates, and analytics visuals.

---

## Tracking

Each epic should have:
- Design doc (short) with acceptance criteria.
- Test plan and sample files.
- Performance budget and measurement notes.
- Schema/versioning notes for export stability where applicable.

If you spot TODOs in code during implementation, create corresponding issues and link them back to the epic. Keep PRs small and focused.
