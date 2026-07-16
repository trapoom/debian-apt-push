
## [1.0.1] - 2026-07-16

### Added
- Integrated a new **Image View** section supporting $512 \times 512$ raw RGB/RGBA byte data rendering.
- Added auto-clearing functionality for the terminal screen upon application startup and exit to prevent display corruption.

### Fixed
- Fixed various compilation warnings regarding unused imports and dead code.
- Cleaned up unconstructed enum variants, unused struct fields (`per_core_usage`, `per_core_temp`, `available_gb`, `buffers_gb`, `cached_gb`, `health`, `technology`), and unused internal functions (`get_cpu_info`, `get_avg_cpu_freq`, `parse_color`).
- Improved overall code readability and performance by eliminating redundant code and resolving type mismatches.

---
