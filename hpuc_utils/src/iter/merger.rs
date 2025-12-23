
pub enum Merger2Way<I1, I2>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
{
    Iter1(I1),
    Iter2(I2),
}

impl<I1, I2> Iterator for Merger2Way<I1, I2>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
{
    type Item = I1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Merger2Way::Iter1(i) => i.next(),
            Merger2Way::Iter2(i) => i.next(),
        }
    }
}

pub trait Merger1Of2<I1>
where
    Self: Iterator + Sized,
    I1: Iterator<Item = Self::Item>,
{
    fn merge_1_of_2(self) -> Merger2Way<Self, I1>;
}

impl<T, I1> Merger1Of2<I1> for T
where
    T: Iterator,
    I1: Iterator<Item = T::Item>,
{
    fn merge_1_of_2(self) -> Merger2Way<Self, I1> {
        Merger2Way::Iter1(self)
    }
}

pub trait Merger2Of2<I1>
where
    Self: Iterator + Sized,
    I1: Iterator<Item = Self::Item>,
{
    fn merge_2_of_2(self) -> Merger2Way<I1, Self>;
}

impl<T, I1> Merger2Of2<I1> for T
where
    T: Iterator,
    I1: Iterator<Item = T::Item>,
{
    fn merge_2_of_2(self) -> Merger2Way<I1, Self> {
        Merger2Way::Iter2(self)
    }
}


pub enum Merger3Way<I1, I2, I3>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    I3: Iterator<Item = I1::Item>,
{
    Iter1(I1),
    Iter2(I2),
    Iter3(I3),
}

impl<I1, I2, I3> Iterator for Merger3Way<I1, I2, I3>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    I3: Iterator<Item = I1::Item>,
{
    type Item = I1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Merger3Way::Iter1(i) => i.next(),
            Merger3Way::Iter2(i) => i.next(),
            Merger3Way::Iter3(i) => i.next(),
        }
    }
}

pub trait Merger1Of3<I1, I2>
where
    Self: Iterator + Sized,
    I1: Iterator<Item = Self::Item>,
    I2: Iterator<Item = Self::Item>,
{
    fn merge_1_of_3(self) -> Merger3Way<Self, I1, I2>;
}

impl<T, I1, I2> Merger1Of3<I1, I2> for T
where
    T: Iterator,
    I1: Iterator<Item = T::Item>,
    I2: Iterator<Item = T::Item>,
{
    fn merge_1_of_3(self) -> Merger3Way<Self, I1, I2> {
        Merger3Way::Iter1(self)
    }
}

pub trait Merger2Of3<I1, I2>
where
    Self: Iterator + Sized,
    I1: Iterator<Item = Self::Item>,
    I2: Iterator<Item = Self::Item>,
{
    fn merge_2_of_3(self) -> Merger3Way<I1, Self, I2>;
}

impl<T, I1, I2> Merger2Of3<I1, I2> for T
where
    T: Iterator,
    I1: Iterator<Item = T::Item>,
    I2: Iterator<Item = T::Item>,
{
    fn merge_2_of_3(self) -> Merger3Way<I1, Self, I2> {
        Merger3Way::Iter2(self)
    }
}

pub trait Merger3Of3<I1, I2>
where
    Self: Iterator + Sized,
    I1: Iterator<Item = Self::Item>,
    I2: Iterator<Item = Self::Item>,
{
    fn merge_3_of_3(self) -> Merger3Way<I1, I2, Self>;
}

impl<T, I1, I2> Merger3Of3<I1, I2> for T
where
    T: Iterator,
    I1: Iterator<Item = T::Item>,
    I2: Iterator<Item = T::Item>,
{
    fn merge_3_of_3(self) -> Merger3Way<I1, I2, Self> {
        Merger3Way::Iter3(self)
    }
}
