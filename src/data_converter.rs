// Probably copied from `RustAudio/rodio/src/conversions/sample.rs`
// Why did I copy & pasted instead of directly depending on it?

use super::seek::{SeekResult, Seekable};
use cpal::{FromSample, Sample as CpalSample};
use rodio::Sample;
use std::marker::PhantomData;

/// Converts the samples data type to `O`.
#[derive(Clone, Debug)]
pub struct DataConverter<I, O> {
    input: I,
    marker: PhantomData<O>,
}

impl<I, O> DataConverter<I, O> {
    /// Builds a new converter.
    #[inline]
    pub fn new(input: I) -> DataConverter<I, O> {
        DataConverter {
            input,
            marker: PhantomData,
        }
    }

    /// Destroys this iterator and returns the underlying iterator.
    #[inline]
    pub fn into_inner(self) -> I {
        self.input
    }

    /// get mutable access to the iterator
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.input
    }
}

impl<I, O> Iterator for DataConverter<I, O>
where
    I: Iterator,
    I::Item: Sample,
    O: FromSample<I::Item> + Sample + CpalSample,
    // In the future version of rodio, Sample: CpalSample
    // so constraints on CpalSample can be removed
{
    type Item = O;

    #[inline]
    fn next(&mut self) -> Option<O> {
        self.input.next().map(|s| CpalSample::from_sample(s))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I, O> ExactSizeIterator for DataConverter<I, O>
where
    I: ExactSizeIterator,
    I::Item: Sample,
    O: FromSample<I::Item> + Sample + CpalSample,
{
}

impl<I, O> Seekable for DataConverter<I, O>
where
    I: Seekable,
{
    fn seek(&mut self, sample: u64) -> SeekResult {
        self.input.seek(sample)
    }
}
