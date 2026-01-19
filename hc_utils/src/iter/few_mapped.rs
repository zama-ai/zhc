use hc_utils_macro::fsm;
use std::{collections::VecDeque};

pub trait IterMapFirst
where
    Self: Iterator + Sized,
{
    fn map_first<'a, A>(self, f: impl FnMut(Self::Item) -> A + 'a) -> MapFirsts<'a, Self, A>;
}
impl<I: Iterator> IterMapFirst for I {
    fn map_first<'a, A>(self, f: impl FnMut(Self::Item) -> A + 'a) -> MapFirsts<'a, Self, A> {
        let mut firsts = VecDeque::new();
        let boxed: Box<dyn FnMut(Self::Item) -> A + 'a> = Box::new(f);
        firsts.push_back(boxed);
        MapFirsts(MapMany::SpecifiedFirsts {
            iter: self,
            firsts,
        })
    }
}

pub struct MapFirsts<'a, I: Iterator, A>(MapMany<'a, I, A>);

impl<'a, I: Iterator, A> MapFirsts<'a, I, A> {
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

pub struct MapFirstsRest<'a, I: Iterator, A>(MapMany<'a, I, A>);

impl<'a, I: Iterator, A> MapFirstsRest<'a, I, A> {
    pub fn map_last(
        self,
        f: impl FnMut(I::Item) -> A + 'a,
    ) -> MapFirstsRestLasts<'a, I, A> {
        let MapFirstsRest(mut mm) = self;
        mm.transition(|old| {
            let MapMany::SpecifiedFirstsRest {
                iter,
                firsts,
                rest,
            } = old
            else {
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

pub struct MapFirstsRestLasts<'a, I: Iterator, A>(MapMany<'a, I, A>);

impl<'a, I: Iterator, A> MapFirstsRestLasts<'a, I, A> {
    pub fn map_last(
        mut self,
        f: impl FnMut(I::Item) -> A + 'a,
    ) -> MapFirstsRestLasts<'a, I, A> {
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

pub trait IterMapRest
where
    Self: Iterator + Sized,
{
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

pub struct MapRest<'a, I: Iterator, A>(MapMany<'a, I, A>);

impl<'a, I: Iterator, A> MapRest<'a, I, A> {
    pub fn map_last(self, f: impl FnMut(I::Item) -> A + 'a) -> MapRestLasts<'a, I, A> {
        let MapRest(mut mm) = self;
        mm.transition(|old| {
            let MapMany::SpecifiedRest { iter, rest } = old else {
                unreachable!();
            };
            let mut lasts = VecDeque::new();
            let boxed: Box<dyn FnMut(I::Item) -> A + 'a> = Box::new(f);
            lasts.push_back(boxed);
            MapMany::SpecifiedRestLasts {
                iter,
                rest,
                lasts,
            }
        });
        MapRestLasts(mm)
    }
}

pub struct MapRestLasts<'a, I: Iterator, A>(MapMany<'a, I, A>);

impl<'a, I: Iterator, A> MapRestLasts<'a, I, A> {
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
            MapMany::SpecifiedRestLasts {
                iter,
                rest,
                lasts,
            }
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
                    panic!("Iterator was not long enough to span the whole map.");
                }
                if firsts.is_empty() {
                    MapMany::RunningOnRestWithoutLasts { iter, has_run_once: false, rest }
                } else {
                    MapMany::RunningOnFirstsWithoutLasts {
                        iter,
                        firsts,
                        rest,
                    }
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
                    panic!("Iterator was not long enough to span the whole map.");
                }
                if firsts.is_empty() {
                    let lookahead: VecDeque<Option<I::Item>> =
                        (0..=lasts.len()).map(|_| iter.next()).collect();
                    assert!(lookahead.iter().all(|l| l.is_some()), "Iterator was not long enough to span the whole map.");
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
                    panic!("Iterator was not long enough to span the whole map.");
                }
                let lookahead: VecDeque<Option<I::Item>> =
                    (0..=lasts.len()).map(|_| iter.next()).collect();
                assert!(lookahead.iter().all(|l| l.is_some()), "Iterator was not long enough to span the whole map.");
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
                    panic!("Iterator was not long enough to span the whole map.");
                }
                if firsts.is_empty() {
                    let lookahead: VecDeque<Option<I::Item>> =
                        (0..=lasts.len()).map(|_| iter.next()).collect();
                    assert!(lookahead.iter().all(|l| l.is_some()), "Iterator was not long enough to span the whole map.");
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
                    panic!("Iterator was not long enough to span the whole map.");
                }
                if firsts.is_empty() {
                    MapMany::RunningOnRestWithoutLasts { iter, has_run_once: false, rest }
                } else {
                    MapMany::RunningOnFirstsWithoutLasts {
                        iter,
                        firsts,
                        rest,
                    }
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
                    panic!("Iterator was not long enough to span the whole map.");
                }
                let look = iter.next();
                if look.is_none() {
                    MapMany::RunningOnLasts {
                        lookahead,
                        lasts,
                    }
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
            MapMany::RunningOnRestWithoutLasts { mut iter, has_run_once, mut rest } => {
                output = iter.next().map(&mut rest);
                if output.is_none() && !has_run_once {
                    panic!("Iterator was not long enough to span the whole map.");
                }
                MapMany::RunningOnRestWithoutLasts { iter, has_run_once: true, rest }
            }

            MapMany::RunningOnLasts {
                mut lookahead,
                mut lasts,
            } => {
                assert_eq!(lookahead.len(), lasts.len());
                output = lookahead
                    .pop_front()
                    .unwrap()
                    .map(lasts.pop_front().unwrap());
                if output.is_none() {
                    panic!("Iterator was not long enough to span the whole map.");
                }
                if lasts.is_empty() {
                    MapMany::Finished
                } else {
                    MapMany::RunningOnLasts {
                        lookahead,
                        lasts,
                    }
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
    #[should_panic(expected = "Iterator was not long enough to span the whole map.")]
    fn test_empty_iterator() {
        let iter = std::iter::empty::<i32>();
        let _mapped: Vec<_> = iter.map_first(|x| x * 2).map_rest(|x| x + 1).collect();
    }

    #[test]
    #[should_panic(expected = "Iterator was not long enough to span the whole map.")]
    fn test_single_item_iterator() {
        let iter = vec![42].into_iter();
        let _mapped: Vec<_> = iter.map_first(|x| x * 2).map_rest(|x| x + 1).collect();
    }
}
