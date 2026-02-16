//! Position-aware iterator mapping with distinct transformations for head, body, and tail.
//!
//! This module provides iterator adapters that apply different mapping functions to different
//! *positions* within a sequence. Unlike the standard [`Iterator::map`], which applies one
//! function uniformly, these adapters let you specify separate transformations for:
//!
//! - **First elements** — one or more leading items, each with its own mapper
//! - **Rest (middle) elements** — the bulk of the sequence, sharing a single mapper
//! - **Last elements** — one or more trailing items, each with its own mapper
//!
//! This is useful when generating output where boundaries need special treatment: adding
//! delimiters, formatting the first line differently, or applying a finalizer to the last item.
//!
//! # Entry Points
//!
//! Two extension traits provide the starting methods on any [`Iterator`]:
//!
//! - [`IterMapFirst::map_first`] — begin by specifying a mapper for the first element
//! - [`IterMapRest::map_rest`] — begin directly with the bulk mapper (no special first handling)
//!
//! From there, a builder-style API lets you chain additional mappers before collecting.
//!
//! # Ordering of Last Mappers
//!
//! When multiple `map_last` calls are chained, they apply to trailing positions in
//! *declaration order*: the first `map_last` handles the element at position `n - k`, the
//! second handles `n - k + 1`, and so on, where `k` is the total number of last mappers.
//!
//! # Example
//!
//! ```rust,no_run
//! # use hc_utils::iter::IterMapFirst;
//! let items = vec![1, 2, 3, 4, 5, 6];
//! let result: Vec<_> = items
//!     .into_iter()
//!     .map_first(|x| x * 2)       // first element: 1 × 2 = 2
//!     .map_first(|x| x * 3)       // second element: 2 × 3 = 6
//!     .map_rest(|x| x + 10)       // middle elements: +10
//!     .map_last(|x| x * 3)        // second-to-last: 5 × 3 = 15
//!     .map_last(|x| x - 5)        // last element: 6 - 5 = 1
//!     .collect();
//! assert_eq!(result, [2, 6, 13, 14, 15, 1]);
//! ```
//!
//! # Short Iterators
//!
//! If the underlying iterator is too short to satisfy the number of distinct mappers, iteration
//! terminates early and returns `None`. In debug builds, a warning is printed to stderr. This
//! allows graceful handling of edge cases while still alerting developers during development.

use hc_utils_macro::fsm;
use std::collections::VecDeque;

/// Emits a warning in debug builds when the iterator is too short.
#[cfg(debug_assertions)]
fn warn_short_iterator() {
    eprintln!("[few_mapped] Warning: iterator was shorter than the number of mappers specified");
}

#[cfg(not(debug_assertions))]
fn warn_short_iterator() {}

/// Extension trait for iterators, providing [`map_first`](Self::map_first).
///
/// This trait is automatically implemented for all types that implement [`Iterator`] and
/// [`Sized`]. Import it to gain access to the `map_first` method on any iterator.
pub trait IterMapFirst
where
    Self: Iterator + Sized,
{
    /// Begins a position-aware mapping chain by specifying a transformation for the first element.
    ///
    /// The closure `f` will be applied exclusively to the first item yielded by this iterator.
    /// Subsequent calls to [`MapFirsts::map_first`] on the returned builder will register
    /// additional mappers for the second, third, etc. elements. Once all leading mappers are
    /// specified, call [`MapFirsts::map_rest`] to define the bulk transformation.
    ///
    /// The returned [`MapFirsts`] is *not* directly iterable — you must finalize the chain by
    /// calling `map_rest` before collecting.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use hc_utils::iter::IterMapFirst;
    /// let nums = vec![10, 20, 30];
    /// let out: Vec<_> = nums
    ///     .into_iter()
    ///     .map_first(|x| x * 100)   // 10 → 1000
    ///     .map_rest(|x| x + 1)      // 20 → 21, 30 → 31
    ///     .collect();
    /// assert_eq!(out, [1000, 21, 31]);
    /// ```
    fn map_first<'a, A>(self, f: impl FnMut(Self::Item) -> A + 'a) -> MapFirsts<'a, Self, A>;
}
impl<I: Iterator> IterMapFirst for I {
    fn map_first<'a, A>(self, f: impl FnMut(Self::Item) -> A + 'a) -> MapFirsts<'a, Self, A> {
        let mut firsts = VecDeque::new();
        let boxed: Box<dyn FnMut(Self::Item) -> A + 'a> = Box::new(f);
        firsts.push_back(boxed);
        MapFirsts(MapMany::SpecifiedFirsts { iter: self, firsts })
    }
}

/// Builder for specifying leading-element mappers before defining the bulk transformation.
///
/// Created by [`IterMapFirst::map_first`]. This type accumulates one mapper per leading position.
/// It is *not* an [`Iterator`] itself — you must call [`map_rest`](Self::map_rest) to finalize
/// the chain and obtain an iterable adapter.
pub struct MapFirsts<'a, I: Iterator, A>(MapMany<'a, I, A>);

impl<'a, I: Iterator, A> MapFirsts<'a, I, A> {
    /// Registers an additional mapper for the next leading element.
    ///
    /// Each call to `map_first` extends the sequence of distinct leading mappers. The first
    /// `map_first` applies to position 0, the second to position 1, and so on.
    pub fn map_first(self, f: impl FnMut(I::Item) -> A + 'a) -> MapFirsts<'a, I, A> {
        let MapFirsts(mut mm) = self;
        mm.transition(|old| {
            let MapMany::SpecifiedFirsts { iter, mut firsts } = old else {
                unreachable!()
            };
            firsts.push_back(Box::new(f));
            MapMany::SpecifiedFirsts { iter, firsts }
        });
        MapFirsts(mm)
    }

    /// Finalizes the leading mappers and specifies the bulk transformation for remaining elements.
    ///
    /// The closure `f` applies to every element after the leading positions. The returned
    /// [`MapFirstsRest`] implements [`Iterator`] and can be collected directly, or you can
    /// continue the chain with [`MapFirstsRest::map_last`] to add trailing-element mappers.
    pub fn map_rest(self, f: impl FnMut(I::Item) -> A + 'a) -> MapFirstsRest<'a, I, A> {
        let MapFirsts(mut mm) = self;
        mm.transition(|old| {
            let MapMany::SpecifiedFirsts { iter, firsts } = old else {
                unreachable!();
            };
            MapMany::SpecifiedFirstsRest {
                iter,
                firsts,
                rest: Box::new(f),
            }
        });
        MapFirstsRest(mm)
    }
}

/// Iterator adapter with distinct mappers for leading elements and a bulk mapper for the rest.
///
/// Created by [`MapFirsts::map_rest`]. This type implements [`Iterator`] and can be collected
/// directly. Optionally, call [`map_last`](Self::map_last) to specify trailing-element mappers
/// before collecting.
pub struct MapFirstsRest<'a, I: Iterator, A>(MapMany<'a, I, A>);

impl<'a, I: Iterator, A> MapFirstsRest<'a, I, A> {
    /// Registers a mapper for a trailing element.
    ///
    /// The first `map_last` call applies to the element at position `n - k`, where `n` is the
    /// iterator length and `k` is the total number of `map_last` mappers that will be registered.
    /// Subsequent `map_last` calls apply to positions `n - k + 1`, `n - k + 2`, etc., with the
    /// final `map_last` handling the very last element.
    pub fn map_last(self, f: impl FnMut(I::Item) -> A + 'a) -> MapFirstsRestLasts<'a, I, A> {
        let MapFirstsRest(mut mm) = self;
        mm.transition(|old| {
            let MapMany::SpecifiedFirstsRest { iter, firsts, rest } = old else {
                unreachable!();
            };
            let mut lasts = VecDeque::new();
            let boxed: Box<dyn FnMut(I::Item) -> A + 'a> = Box::new(f);
            lasts.push_back(boxed);
            MapMany::SpecifiedFirstsRestLasts {
                iter,
                firsts,
                rest,
                lasts,
            }
        });
        MapFirstsRestLasts(mm)
    }
}

impl<'a, I: Iterator, A> Iterator for MapFirstsRest<'a, I, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Iterator adapter with distinct mappers for leading, middle, and trailing elements.
///
/// Created by [`MapFirstsRest::map_last`]. This type implements [`Iterator`] and can be
/// collected directly. You may continue chaining [`map_last`](Self::map_last) to register
/// additional trailing mappers.
pub struct MapFirstsRestLasts<'a, I: Iterator, A>(MapMany<'a, I, A>);

impl<'a, I: Iterator, A> MapFirstsRestLasts<'a, I, A> {
    /// Registers an additional mapper for the next trailing position.
    ///
    /// Each `map_last` call extends the trailing region by one element. Mappers apply in
    /// declaration order: the first registered handles the earliest trailing position, the
    /// last registered handles the final element.
    pub fn map_last(mut self, f: impl FnMut(I::Item) -> A + 'a) -> MapFirstsRestLasts<'a, I, A> {
        self.0.transition(|old| {
            let MapMany::SpecifiedFirstsRestLasts {
                iter,
                firsts,
                rest,
                mut lasts,
            } = old
            else {
                unreachable!();
            };
            lasts.push_back(Box::new(f));
            MapMany::SpecifiedFirstsRestLasts {
                iter,
                firsts,
                rest,
                lasts,
            }
        });
        self
    }
}

impl<'a, I: Iterator, A> Iterator for MapFirstsRestLasts<'a, I, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Extension trait for iterators, providing [`map_rest`](Self::map_rest).
///
/// This trait is automatically implemented for all types that implement [`Iterator`] and
/// [`Sized`]. Import it to gain access to the `map_rest` method on any iterator.
///
/// Use this entry point when you do not need distinct mappers for leading elements — only a
/// bulk mapper and optionally trailing mappers.
pub trait IterMapRest
where
    Self: Iterator + Sized,
{
    /// Begins a position-aware mapping chain with a bulk transformation.
    ///
    /// The closure `f` applies to all elements except those later designated as trailing via
    /// [`MapRest::map_last`]. If no `map_last` calls follow, `f` applies to the entire sequence.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use hc_utils::iter::IterMapRest;
    /// let nums = vec![1, 2, 3, 4];
    /// let out: Vec<_> = nums
    ///     .into_iter()
    ///     .map_rest(|x| x + 10)     // bulk: +10
    ///     .map_last(|x| x * 100)    // last element: 4 × 100 = 400
    ///     .collect();
    /// assert_eq!(out, [11, 12, 13, 400]);
    /// ```
    fn map_rest<'a, A>(self, f: impl FnMut(Self::Item) -> A + 'a) -> MapRest<'a, Self, A>;
}
impl<I: Iterator> IterMapRest for I {
    fn map_rest<'a, A>(self, f: impl FnMut(Self::Item) -> A + 'a) -> MapRest<'a, Self, A> {
        MapRest(MapMany::SpecifiedRest {
            iter: self,
            rest: Box::new(f),
        })
    }
}

/// Iterator adapter with a bulk mapper, optionally extended with trailing mappers.
///
/// Created by [`IterMapRest::map_rest`]. This type is *not* directly iterable — if you need
/// trailing mappers, call [`map_last`](Self::map_last); otherwise, use standard [`Iterator::map`]
/// instead of this module.
pub struct MapRest<'a, I: Iterator, A>(MapMany<'a, I, A>);

impl<'a, I: Iterator, A> MapRest<'a, I, A> {
    /// Registers a mapper for a trailing element.
    ///
    /// The first `map_last` call applies to the element at position `n - k`, where `n` is the
    /// iterator length and `k` is the total number of `map_last` mappers that will be registered.
    /// Subsequent `map_last` calls apply to later positions, with the final call handling the
    /// very last element.
    pub fn map_last(self, f: impl FnMut(I::Item) -> A + 'a) -> MapRestLasts<'a, I, A> {
        let MapRest(mut mm) = self;
        mm.transition(|old| {
            let MapMany::SpecifiedRest { iter, rest } = old else {
                unreachable!();
            };
            let mut lasts = VecDeque::new();
            let boxed: Box<dyn FnMut(I::Item) -> A + 'a> = Box::new(f);
            lasts.push_back(boxed);
            MapMany::SpecifiedRestLasts { iter, rest, lasts }
        });
        MapRestLasts(mm)
    }
}

/// Iterator adapter with a bulk mapper and one or more trailing mappers.
///
/// Created by [`MapRest::map_last`]. This type implements [`Iterator`] and can be collected
/// directly. You may continue chaining [`map_last`](Self::map_last) to register additional
/// trailing mappers.
pub struct MapRestLasts<'a, I: Iterator, A>(MapMany<'a, I, A>);

impl<'a, I: Iterator, A> MapRestLasts<'a, I, A> {
    /// Registers an additional mapper for the next trailing position.
    ///
    /// Each `map_last` call extends the trailing region by one element. Mappers apply in
    /// declaration order: the first registered handles the earliest trailing position, the
    /// last registered handles the final element.
    pub fn map_last(mut self, f: impl FnMut(I::Item) -> A + 'a) -> MapRestLasts<'a, I, A> {
        self.0.transition(|old| {
            let MapMany::SpecifiedRestLasts {
                iter,
                rest,
                mut lasts,
            } = old
            else {
                unreachable!();
            };
            lasts.push_back(Box::new(f));
            MapMany::SpecifiedRestLasts { iter, rest, lasts }
        });
        self
    }
}

impl<'a, I: Iterator, A> Iterator for MapRestLasts<'a, I, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[fsm]
enum MapMany<'a, I: Iterator, A> {
    SpecifiedFirsts {
        iter: I,
        firsts: VecDeque<Box<dyn FnMut(I::Item) -> A + 'a>>,
    },
    SpecifiedFirstsRest {
        iter: I,
        firsts: VecDeque<Box<dyn FnMut(I::Item) -> A + 'a>>,
        rest: Box<dyn FnMut(I::Item) -> A + 'a>,
    },
    SpecifiedFirstsRestLasts {
        iter: I,
        firsts: VecDeque<Box<dyn FnMut(I::Item) -> A + 'a>>,
        rest: Box<dyn FnMut(I::Item) -> A + 'a>,
        lasts: VecDeque<Box<dyn FnMut(I::Item) -> A + 'a>>,
    },
    SpecifiedRest {
        iter: I,
        rest: Box<dyn FnMut(I::Item) -> A + 'a>,
    },
    SpecifiedRestLasts {
        iter: I,
        rest: Box<dyn FnMut(I::Item) -> A + 'a>,
        lasts: VecDeque<Box<dyn FnMut(I::Item) -> A + 'a>>,
    },
    RunningOnFirsts {
        iter: I,
        firsts: VecDeque<Box<dyn FnMut(I::Item) -> A + 'a>>,
        rest: Box<dyn FnMut(I::Item) -> A + 'a>,
        lasts: VecDeque<Box<dyn FnMut(I::Item) -> A + 'a>>,
    },
    RunningOnFirstsWithoutLasts {
        iter: I,
        firsts: VecDeque<Box<dyn FnMut(I::Item) -> A + 'a>>,
        rest: Box<dyn FnMut(I::Item) -> A + 'a>,
    },
    RunningOnRest {
        iter: I,
        lookahead: VecDeque<Option<I::Item>>,
        rest: Box<dyn FnMut(I::Item) -> A + 'a>,
        lasts: VecDeque<Box<dyn FnMut(I::Item) -> A + 'a>>,
    },
    RunningOnRestWithoutLasts {
        iter: I,
        has_run_once: bool,
        rest: Box<dyn FnMut(I::Item) -> A + 'a>,
    },
    RunningOnLasts {
        lookahead: VecDeque<Option<I::Item>>,
        lasts: VecDeque<Box<dyn FnMut(I::Item) -> A + 'a>>,
    },
    Finished,
}

impl<'a, I: Iterator, A> Iterator for MapMany<'a, I, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = None;
        self.transition(|old| match old {
            MapMany::SpecifiedFirstsRest {
                mut iter,
                mut firsts,
                rest,
            } => {
                let mapper = firsts.pop_front().unwrap();
                output = iter.next().map(mapper);
                if output.is_none() {
                    warn_short_iterator();
                    return MapMany::Finished;
                }
                if firsts.is_empty() {
                    MapMany::RunningOnRestWithoutLasts {
                        iter,
                        has_run_once: false,
                        rest,
                    }
                } else {
                    MapMany::RunningOnFirstsWithoutLasts { iter, firsts, rest }
                }
            }
            MapMany::SpecifiedFirstsRestLasts {
                mut iter,
                mut firsts,
                rest,
                lasts,
            } => {
                let mapper = firsts.pop_front().unwrap();
                output = iter.next().map(mapper);
                if output.is_none() {
                    warn_short_iterator();
                    return MapMany::Finished;
                }
                if firsts.is_empty() {
                    let lookahead: VecDeque<Option<I::Item>> =
                        (0..=lasts.len()).map(|_| iter.next()).collect();
                    if !lookahead.iter().all(|l| l.is_some()) {
                        warn_short_iterator();
                        return MapMany::Finished;
                    }
                    MapMany::RunningOnRest {
                        iter,
                        lookahead,
                        rest,
                        lasts,
                    }
                } else {
                    MapMany::RunningOnFirsts {
                        iter,
                        firsts,
                        rest,
                        lasts,
                    }
                }
            }
            MapMany::SpecifiedRestLasts {
                mut iter,
                mut rest,
                lasts,
            } => {
                output = iter.next().map(&mut rest);
                if output.is_none() {
                    warn_short_iterator();
                    return MapMany::Finished;
                }
                let lookahead: VecDeque<Option<I::Item>> =
                    (0..=lasts.len()).map(|_| iter.next()).collect();
                if !lookahead.iter().all(|l| l.is_some()) {
                    warn_short_iterator();
                    return MapMany::Finished;
                }
                MapMany::RunningOnRest {
                    iter,
                    lookahead,
                    rest,
                    lasts,
                }
            }
            MapMany::RunningOnFirsts {
                mut iter,
                mut firsts,
                rest,
                lasts,
            } => {
                output = iter.next().map(firsts.pop_front().unwrap());
                if output.is_none() {
                    warn_short_iterator();
                    return MapMany::Finished;
                }
                if firsts.is_empty() {
                    let lookahead: VecDeque<Option<I::Item>> =
                        (0..=lasts.len()).map(|_| iter.next()).collect();
                    if !lookahead.iter().all(|l| l.is_some()) {
                        warn_short_iterator();
                        return MapMany::Finished;
                    }
                    MapMany::RunningOnRest {
                        iter,
                        lookahead,
                        rest,
                        lasts,
                    }
                } else {
                    MapMany::RunningOnFirsts {
                        iter,
                        firsts,
                        rest,
                        lasts,
                    }
                }
            }
            MapMany::RunningOnFirstsWithoutLasts {
                mut iter,
                mut firsts,
                rest,
            } => {
                output = iter.next().map(firsts.pop_front().unwrap());
                if output.is_none() {
                    warn_short_iterator();
                    return MapMany::Finished;
                }
                if firsts.is_empty() {
                    MapMany::RunningOnRestWithoutLasts {
                        iter,
                        has_run_once: false,
                        rest,
                    }
                } else {
                    MapMany::RunningOnFirstsWithoutLasts { iter, firsts, rest }
                }
            }
            MapMany::RunningOnRest {
                mut iter,
                mut lookahead,
                mut rest,
                lasts,
            } => {
                output = lookahead.pop_front().unwrap().map(&mut rest);
                if output.is_none() {
                    warn_short_iterator();
                    return MapMany::Finished;
                }
                let look = iter.next();
                if look.is_none() {
                    MapMany::RunningOnLasts { lookahead, lasts }
                } else {
                    lookahead.push_back(look);
                    MapMany::RunningOnRest {
                        iter,
                        rest,
                        lookahead,
                        lasts,
                    }
                }
            }
            MapMany::RunningOnRestWithoutLasts {
                mut iter,
                has_run_once,
                mut rest,
            } => {
                output = iter.next().map(&mut rest);
                if output.is_none() && !has_run_once {
                    warn_short_iterator();
                }
                MapMany::RunningOnRestWithoutLasts {
                    iter,
                    has_run_once: true,
                    rest,
                }
            }

            MapMany::RunningOnLasts {
                mut lookahead,
                mut lasts,
            } => {
                debug_assert_eq!(lookahead.len(), lasts.len());
                output = lookahead
                    .pop_front()
                    .unwrap()
                    .map(lasts.pop_front().unwrap());
                if output.is_none() {
                    warn_short_iterator();
                    return MapMany::Finished;
                }
                if lasts.is_empty() {
                    MapMany::Finished
                } else {
                    MapMany::RunningOnLasts { lookahead, lasts }
                }
            }
            MapMany::Finished => {
                output = None;
                MapMany::Finished
            }
            _ => unreachable!(),
        });
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_prelude_single() {
        let iter = vec![1, 2, 3].into_iter();
        let mapped: Vec<_> = iter.map_first(|x| x * 7).map_rest(|x| x + 1).collect();
        assert_eq!(mapped, vec![7, 3, 4])
    }

    #[test]
    fn test_map_prelude_multiple() {
        let iter = vec![1, 2, 3, 4].into_iter();
        let mapped: Vec<_> = iter
            .map_first(|x| x * 2)
            .map_first(|x| x * 3)
            .map_rest(|x| x + 1)
            .collect();
        assert_eq!(mapped, [2, 6, 4, 5]);
    }

    #[test]
    fn test_map_prelude_bulk_postlude() {
        let iter = vec![1, 2, 3, 4, 5].into_iter();
        let mapped: Vec<_> = iter
            .map_first(|x| x * 2)
            .map_rest(|x| x + 10)
            .map_last(|x| x * 3)
            .collect();
        assert_eq!(mapped, vec![2, 12, 13, 14, 15])
    }

    #[test]
    fn test_map_prelude_bulk_multiple_postlude() {
        let iter = vec![1, 2, 3, 4, 5, 6].into_iter();
        let mapped: Vec<_> = iter
            .map_first(|x| x * 2)
            .map_rest(|x| x + 10)
            .map_last(|x| x * 3)
            .map_last(|x| x - 5)
            .collect();
        assert_eq!(mapped, [2, 12, 13, 14, 15, 1]);
    }

    #[test]
    fn test_empty_iterator() {
        let iter = std::iter::empty::<i32>();
        let mapped: Vec<_> = iter.map_first(|x| x * 2).map_rest(|x| x + 1).collect();
        assert!(mapped.is_empty());
    }

    #[test]
    fn test_single_item_iterator() {
        let iter = vec![42].into_iter();
        let mapped: Vec<_> = iter.map_first(|x| x * 2).map_rest(|x| x + 1).collect();
        // Only the first mapper runs; "rest" never gets an element
        assert_eq!(mapped, [84]);
    }
}
