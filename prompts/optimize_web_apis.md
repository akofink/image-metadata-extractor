# Web API Optimization Workstream

## Context

This project is a Rust + WebAssembly image metadata extraction tool that runs entirely in the browser. We've identified opportunities to replace manual implementations with native Web APIs for better performance, smaller bundle size, and improved UX.

See `TODO_web_APIs.md` in the project root for detailed optimization opportunities and implementation strategies.

## Goals

1. **Performance**: Reduce perceived load time and improve responsiveness
2. **Bundle Size**: Remove redundant code that's available natively in browsers
3. **UX**: Provide better download experiences and smoother interactions
4. **Maintainability**: Replace custom implementations with well-tested browser APIs

## Current Status

**Completed Optimizations**:
- ✅ Replaced base64 data URLs with blob object URLs (85-90% memory reduction)

**Planned Optimizations** (see TODO_web_APIs.md):
1. 🎯 Remove unused `base64_encode` function
2. 🎯 Use `requestIdleCallback()` for hash calculation
3. 🎯 Implement Web Workers for batch processing
4. 📊 Use `showSaveFilePicker()` for better download UX
5. 📊 Consider `CompressionStream` for ZIP creation
6. 💡 Evaluate `OffscreenCanvas` for image dimensions

## Implementation Approach

### Phase 1: Quick Wins (Current Phase)
**Target**: Low-hanging fruit with immediate impact

**Tasks**:
1. Remove unused `base64_encode` function from `src/utils_wasm.rs`
2. Implement `requestIdleCallback()` wrapper for hash calculation
3. Update hash calculation calls to use idle callback

**Success Criteria**:
- All tests pass
- Bundle size reduced
- Hash calculation doesn't block UI rendering
- Fallback exists for browsers without `requestIdleCallback`

### Phase 2: UX Improvements
**Target**: Better download experience

**Tasks**:
1. Implement `showSaveFilePicker()` with feature detection
2. Add fallback to current anchor-based download method
3. Update download functions in `src/utils_wasm.rs`
4. Test across browsers (Chrome, Firefox, Safari)

**Success Criteria**:
- Users can choose download location on supported browsers
- Graceful fallback on unsupported browsers
- No regression in download functionality

### Phase 3: Performance Overhaul
**Target**: Dramatically improve batch processing

**Tasks**:
1. Research wasm-bindgen worker setup
2. Create worker module for heavy operations
3. Move EXIF extraction to worker
4. Move hash calculation to worker
5. Implement progress reporting
6. Add error handling and fallbacks

**Success Criteria**:
- Batch processing 50-200% faster
- UI stays responsive during processing
- Progress indicators work correctly
- Graceful degradation if workers unavailable

## Key Files

**Core Files to Modify**:
- `src/utils_wasm.rs` - Utility functions using Web APIs
- `src/utils_hash.rs` - SHA-256 hash calculation
- `src/exif_wasm.rs` - EXIF extraction (WASM-specific)
- `src/components/file_upload.rs` - File upload and batch processing
- `src/components/batch_cleaner.rs` - Batch ZIP creation

**Testing Files**:
- Add new tests for feature detection and fallbacks
- Update existing tests to mock Web APIs
- Add performance benchmarks

## Browser Support Requirements

Always implement with progressive enhancement:
- **Chrome/Edge**: Target latest 2 versions
- **Firefox**: Target latest 2 versions
- **Safari**: Target latest 2 versions
- **Mobile**: Test on iOS Safari and Chrome Android

**Feature Detection Pattern**:
```rust
// Check if API exists
if js_sys::Reflect::has(&window, &JsValue::from_str("requestIdleCallback"))? {
    // Use modern API
} else {
    // Fallback to traditional approach
}
```

## Testing Strategy

For each Web API optimization:

1. **Unit Tests**: Test API wrappers in isolation
2. **Integration Tests**: Test full workflow with new APIs
3. **Browser Tests**: Manual testing across browsers
4. **Performance Tests**: Benchmark before/after
5. **Fallback Tests**: Verify graceful degradation

**Performance Metrics to Track**:
- Time to first interaction
- Hash calculation time
- Batch processing time
- Bundle size (WASM + JS)
- Memory usage during batch operations

## References

- **TODO File**: `TODO_web_APIs.md` - Detailed optimization catalog
- **MDN Web APIs**: https://developer.mozilla.org/en-US/docs/Web/API
- **web-sys Docs**: https://rustwasm.github.io/wasm-bindgen/api/web_sys/
- **Can I Use**: https://caniuse.com/ - Browser support tables

## Notes for Claude Code

When working on this optimization stream:

1. **Always check browser support** before implementing
2. **Provide fallbacks** for newer APIs
3. **Measure before optimizing** - run benchmarks
4. **Test on real devices** - mobile especially important
5. **Update TODO_web_APIs.md** as work progresses
6. **Keep bundle size in mind** - monitor with each change
7. **Follow progressive enhancement** - start with feature detection

## Quick Start

To continue this work:

```bash
# Review the TODO file
cat TODO_web_APIs.md

# Start with Phase 1 quick wins
# 1. Remove unused base64_encode function
# 2. Implement requestIdleCallback for hash calculation

# Run tests after each change
make test

# Check bundle size
make build-release
ls -lh pkg/*.wasm
```

## Success Indicators

This optimization work is successful when:

1. ✅ Bundle size is reduced (target: -5-10% after all phases)
2. ✅ Perceived performance improves (faster time to interaction)
3. ✅ Batch operations are noticeably faster
4. ✅ UI stays responsive during heavy operations
5. ✅ Download UX is better on supported browsers
6. ✅ All tests pass across all browsers
7. ✅ Code is simpler and more maintainable
