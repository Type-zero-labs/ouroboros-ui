//! Semantic tokens — the shadcn vocabulary mapped onto [`core`](super::core).
//!
//! The [`Theme`] struct holds the resolved semantic tokens (`background`/`foreground`
//! pairs, `primary`, `muted`, `accent`, `destructive`, `border`, `ring`, …) plus the
//! domain semantics (`success`/`warning`/`error`/`info`). Populated in T03.

/// Resolved semantic theme — every color/typography token the design system exposes.
///
/// Fields are filled in T03 (semantic → core mapping decided token-by-token).
#[derive(Clone, Debug, Default)]
pub struct Theme {
    // Populated in T03.
}
