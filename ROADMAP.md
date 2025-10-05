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
- **Archive & dataset native**: Import ZIP/7z/TAR archives, deduplicate by hash, and process at scale without uploading files anywhere.
- **Cross‑file intelligence**: Timeline view, location clustering heatmap, and cross‑image diff/dedupe to analyze entire shoots.
- **Privacy score & guidance**: Automatic sensitive‑field warnings, GPS precision fuzzing, and share‑safety checklist with clear remediation.
- **Templated reporting**: Custom, user‑defined report templates (Markdown/HTML/TXT) with local profile storage and schema versioning.
- **Minimal video metadata**: Parse container metadata (MP4/MOV) and embedded XMP/EXIF sidecar basics to extend usefulness beyond still images.
- **Power‑user experience**: Command palette, keyboard shortcuts, rule‑based redaction engine, and offline quick‑map preview (no external tiles).
- **Internationalization**: Built‑in multilingual UI and field explanations, shipped with the app (no network fetch).
- **Forensic analysis suite**: Error Level Analysis (ELA), timestamp anomaly detection, camera fingerprinting, and provenance tracking.
- **AI-powered insights**: Automated privacy risk assessment, suspicious modification detection, and smart field categorization.
- **Professional workflow integration**: Export profiles for journalism, law enforcement, real estate, and research use cases.
- **Advanced visualization**: Interactive GPS tracks, photo timeline correlation, and metadata trend analysis across collections.
- **Enterprise features**: Audit trails, compliance reporting, and bulk processing with customizable rules engines.

---

## Phase 0 — Immediate Impact Features (Next Priority)

Objectives: Implement high-value features that differentiate us from competitors immediately, focusing on export flexibility and privacy features.

### 🛡️ CRITICAL SECURITY & TRUST ISSUE (URGENT)

- **Complete HEIC/HEIF Cleaning Support** — HIGH PRIORITY SECURITY FIX
  - **TIFF Status**: ✅ **COMPLETED** - Now fully supported with tiff crate implementation
  - **HEIC/HEIF Status**: ❌ **Still needs implementation** - Currently returns honest error messages
  - **Current Issue**: HEIC/HEIF formats show error messages instead of cleaning (good for security)
  - **User Problem**: Apple users with HEIC photos cannot clean metadata 
  - **Privacy Impact**: iPhone/iPad users stuck with privacy-exposing metadata in photos
  - **Implementation Challenge**: libheif-rs doesn't compile for WebAssembly target
  - **Solution Required**: Research WebAssembly-compatible HEIF parsing solutions
  - **Implementation Notes**: 
    - ✅ TIFF: Complete - uses tiff crate for decode/re-encode without metadata
    - ❌ HEIC/HEIF: Need WebAssembly-compatible HEIF box structure parsing
    - Consider pure Rust HEIF implementation or WebAssembly-compatible wrapper
  - **Success Criteria**: HEIC/HEIF cleaning works in browser without native dependencies

User‑facing features:
- **Export enhancements** (PRIORITY):
  - ✅ **Batch export combined metadata**: **IMPLEMENTED** - Single JSON/CSV/TXT containing all processed files
  - ✅ **Copy to clipboard**: **IMPLEMENTED** - Direct copy for JSON/CSV/TXT/MD/YAML/XML metadata without downloads
  - ✅ **Export profiles**: **IMPLEMENTED** - Save/load selection patterns with 5 preset profiles (journalism, real estate, forensics, privacy-safe, research/technical) and custom profile management
  - ✅ **YAML and XML export**: **IMPLEMENTED** - Additional structured formats for professional workflows
- **GPS privacy & security**:
  - ✅ **GPS precision degradation**: **IMPLEMENTED** - Fuzz coordinates to configurable precision levels (exact, street, neighborhood, city, region)
  - ✅ **Privacy risk scoring**: **IMPLEMENTED** - Automatic risk assessment with 4 levels (Low/Medium/High/Critical), detects sensitive fields (GPS, serial numbers, owner names, timestamps), and provides actionable privacy warnings
  - ✅ **Map link generation**: **IMPLEMENTED** - One-click copy of Google/Apple/OSM map links (no external requests)
- **File integrity & forensics**:
  - ✅ **SHA-256 file hashing**: **IMPLEMENTED** - Generate checksums for provenance and deduplication
  - ✅ **Metadata consistency checks**: **IMPLEMENTED** - Automatic validation of timestamp inconsistencies, GPS reference fields, dimension mismatches, and incomplete metadata patterns with visual warnings
  - ✅ **Duplicate detection**: **IMPLEMENTED** - Identify identical files in batch uploads by hash with visual warnings
- **UX improvements**:
  - ✅ **Command palette**: **IMPLEMENTED** - Keyboard-driven interface (Ctrl+K/Cmd+K) for power users with commands for theme, export, copy, and metadata selection.
  - ✅ **Persistent preferences**: **IMPLEMENTED** - Remember theme, export settings, and field selections
  - ✅ **Keyboard shortcuts**: **IMPLEMENTED** - Shortcuts for opening the command palette, file dialog, image modal, toggling explanations, selecting/deselecting metadata, and exporting/copying metadata.

Technical notes:
- Focus on features that can be implemented without Web Workers initially
- Use localStorage for preferences and export profiles
- Implement SHA-256 hashing using Web Crypto API
- GPS fuzzing algorithms using coordinate math (no external services)

Testing & acceptance:
- Export functionality works across all supported formats
- Privacy features effectively reduce location precision
- Keyboard navigation works smoothly across all components
- Settings persist across browser sessions

---

## Phase 1 — Quality, UX, and Export Depth

Objectives: improve the core experience, expand export options, and enable multi‑file workflows while keeping memory use and performance tight.

User‑facing features:
- ✅ **Batch support (multi file)**: **IMPLEMENTED**
  - ✅ Drag‑and‑drop multiple images; process sequentially with progress reporting.
  - ✅ Previous/Next navigation buttons for switching between processed files.
  - ✅ Export combined metadata (single JSON/CSV/TXT) and per‑file outputs — basic combined export IMPLEMENTED (JSON array, CSV table, TXT concat).
  - ✅ Batch cleaning with ZIP download of cleaned files — IMPLEMENTED using zip crate with deflate compression.
- ✅ **Archive input**: **IMPLEMENTED**
  - ✅ Import ZIP archives directly with automatic image file detection and extraction.
  - ✅ Progress tracking during archive extraction and processing.
  - ✅ Streamed extraction in WASM with client-side processing (no server uploads).
  - 🔄 TAR format support (pending).
- Export enhancements:
  - Additional formats: YAML, XML, and Markdown report.
  - Templated reports: user‑defined templates for Markdown/HTML/TXT with local persistence.
  - Per‑category export profiles (save/load locally) for repeatable selections.
  - Copy to clipboard for JSON/CSV/TXT/Markdown.
  - Stable schemas: include schema version, deterministic ordering, and optional JSON‑LD context.
- UX polish:
  - ✅ **Dark mode and system theme auto‑detection**: **IMPLEMENTED**
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
- Use `requestIdleCallback` for deferred computation to keep UI responsive
- Client‑side ZIP creation for batch export/clean using a WASM‑friendly crate (already implemented)
- Archive readers (ZIP/TAR) compiled to WASM; stream via Blob.slice and async iterators
- Avoid large in‑memory spikes by streaming reads and chunked processing
- Web Workers deferred: complexity outweighs benefits for current use cases (see TODO_web_APIs.md)

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
- IndexedDB abstraction for safe, bounded storage (eviction strategy, quotas)
- Rule engine for cleaning: compile a set of predicates to include/exclude/transform keys
- Minimal EXIF write path for limited, safe tag updates; robust validation to avoid corrupting files
- WASM threads (via SharedArrayBuffer) may be considered in future if clear performance bottlenecks emerge

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

## Phase 3.5 — AI-Powered Forensics & Intelligence (BREAKTHROUGH FEATURES)

Objectives: Implement cutting-edge features that no competitor offers, positioning us as the definitive metadata intelligence platform.

User‑facing features:
- **AI-powered analysis** (using client-side ML models via WebAssembly):
  - Smart privacy risk assessment: Automatically detect and score privacy-sensitive patterns
  - Suspicious modification detection: Flag potential image manipulations using metadata patterns
  - Camera fingerprinting: Identify unique camera signatures across image collections
  - Automated field categorization: Intelligently organize custom/unknown metadata fields
- **Advanced forensics suite**:
  - Error Level Analysis (ELA): Detect image compression inconsistencies indicating modifications
  - Timestamp anomaly detection: Flag suspicious date/time patterns across batch uploads
  - Metadata provenance tracking: Build chains of custody and modification history
  - Cross-image correlation: Detect relationships between images in large datasets
- **Professional intelligence features**:
  - **Journalism mode**: Flag potentially sensitive source-revealing metadata
  - **Legal discovery mode**: Generate chain-of-custody reports with audit trails
  - **Research mode**: Statistical analysis and pattern detection across datasets  
  - **OSINT mode**: Optimize for open-source intelligence gathering workflows
- **Advanced visualization dashboard**:
  - Interactive GPS track reconstruction from photo sequences
  - Timeline correlation with automatic clustering by location/time
  - Metadata trend analysis with statistical charts and anomaly highlighting
  - Cross-collection comparison and similarity analysis

Technical notes:
- Client-side ML models using WASM builds of TensorFlow Lite or ONNX Runtime
- ELA implementation using canvas-based image recompression and pixel difference analysis
- Statistical analysis using Rust-compiled math libraries for performance
- All AI/ML processing remains entirely client-side for privacy

Testing & acceptance:
- AI models achieve >90% accuracy on standard forensic test datasets
- ELA analysis matches or exceeds desktop tools like FotoForensics
- Professional workflow modes generate industry-standard report formats
- Performance remains responsive even with ML analysis on mobile devices

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
- `requestIdleCallback` for deferred computation to prevent UI blocking (already implemented)
- Object URLs for memory-efficient image handling (already implemented)
- Progressive enhancement with modern Web APIs (File System Access API for downloads)
- SIMD for hot paths if it materially improves performance
- Deterministic outputs and stable sort orders for reproducible exports
- WASM threads deferred until clear performance bottlenecks identified

Internationalization:
- Localized UI and field explanations.
- Locale switcher with persisted preference; no runtime network fetches.
- Right‑to‑left layout support where applicable.

Developer experience:
- ✅ **E2E testing with Playwright**: **HIGH PRIORITY** - Prevent feature regressions with comprehensive browser tests
- Expand wasm‑bindgen tests covering UI logic in components
- Add property‑based tests for parsing edge cases
- Tooling tasks: bundle size check, flamegraph‑like profiling notes for WASM
- JSON schema definitions for exports with versioning and compatibility tests

---

## Milestones Overview

- **M0 (Phase 0)**: Export enhancements, GPS privacy features, file integrity, command palette, persistent preferences.  
  **Done when**: Clipboard export works, GPS fuzzing implemented, SHA-256 hashing active, keyboard shortcuts functional.

- **M1 (Phase 1)**: Archive import, templated reports, advanced export formats, full i18n support, integrity checks.
  **Status**: Batch ZIP cleaning ✅ complete. ZIP archive import ✅ complete. Remaining: Templated reports, i18n, TAR support.
  **Done when**: Templated reports functional, multilingual UI ships, TAR import added.

- **M2 (Phase 2)**: PWA installation, IndexedDB persistence, directory import, advanced cleaning controls, diff views.  
  **Done when**: Offline-capable installable app, session resume works, granular redaction rules tested.

- **M3 (Phase 3)**: IPTC/XMP parsing, cross‑file analytics, timeline views, location clustering, privacy scoring.  
  **Done when**: Comprehensive metadata parsing, visual analytics dashboard, automated privacy warnings.

- **M3.5 (Phase 3.5)**: AI-powered forensics, ELA analysis, professional workflow modes, advanced intelligence features.  
  **Done when**: Client-side ML models functional, forensic analysis matches desktop tools, professional reports generated.

- **M4 (Phase 4)**: HEIC/HEIF/RAW support, video metadata, WebExtension packaging, enterprise distribution.  
  **Done when**: Capability-gated builds ship, browser extension published, enterprise features tested.

---

## Implementation Hints (Codebase Mapping)

- UI components (`src/components/`):
  - New components: `batch_manager.rs`, `archive_import.rs`, `preferences.rs`, `diff_view.rs`, `rules_editor.rs`, `timeline.rs`, `map_preview.rs`, `command_palette.rs`.
  - Extend existing `metadata_export.rs` for new formats, templates, clipboard support, and schema versioning.
- Core logic:
  - `exif_core.rs` and `metadata_info.rs`: extend to include IPTC/XMP, rules engine, integrity checks, and suggestions/privacy score.
  - `binary_cleaner.rs`: support selective redaction, timestamp shift, and limited safe write‑back.
  - `export.rs`: templating engine, JSON‑LD/context, and schema stability.
  - `utils_wasm.rs`: File System Access API, clipboard, file streams, hashing, `requestIdleCallback`.
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
