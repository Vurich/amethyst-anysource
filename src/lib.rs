use amethyst::{assets::Source, Error};
use std::{io, iter::FromIterator};

pub struct None;
pub struct EitherSource<A, B>(A, B);
pub struct ManySources<S>(Vec<S>);

impl Source for None {
    fn modified(&self, path: &str) -> Result<u64, Error> {
        Err(Error::new(io::Error::from(io::ErrorKind::NotFound)))
    }

    fn load(&self, path: &str) -> Result<Vec<u8>, Error> {
        Err(Error::new(io::Error::from(io::ErrorKind::NotFound)))
    }
}

impl<A: Source, B: Source> Source for EitherSource<A, B> {
    fn modified(&self, path: &str) -> Result<u64, Error> {
        self.0.modified(path).or_else(|_| self.1.modified(path))
    }

    fn load(&self, path: &str) -> Result<Vec<u8>, Error> {
        self.0.load(path).or_else(|_| self.1.load(path))
    }

    fn load_with_metadata(&self, path: &str) -> Result<(Vec<u8>, u64), Error> {
        self.0
            .load_with_metadata(path)
            .or_else(|_| self.1.load_with_metadata(path))
    }
}

impl<S: Source> Source for ManySources<S> {
    fn modified(&self, path: &str) -> Result<u64, Error> {
        for source in &self.0 {
            if let Ok(modified) = source.modified(path) {
                return Ok(modified);
            }
        }

        Err(Error::new(io::Error::from(io::ErrorKind::NotFound)))
    }

    fn load(&self, path: &str) -> Result<Vec<u8>, Error> {
        for source in &self.0 {
            if let Ok(out) = source.load(path) {
                return Ok(out);
            }
        }

        Err(Error::new(io::Error::from(io::ErrorKind::NotFound)))
    }

    fn load_with_metadata(&self, path: &str) -> Result<(Vec<u8>, u64), Error> {
        for source in &self.0 {
            if let Ok(out) = source.load_with_metadata(path) {
                return Ok(out);
            }
        }

        Err(Error::new(io::Error::from(io::ErrorKind::NotFound)))
    }
}

pub struct AnySource<S> {
    sources: S,
}

impl AnySource<None> {
    pub fn new() -> Self {
        AnySource { sources: None }
    }
}

impl<S: Source> AnySource<S> {
    pub fn with_source<A: Source>(mut self, source: A) -> AnySource<impl Source> {
        AnySource {
            sources: EitherSource(source, self.sources),
        }
    }

    pub fn with_sources<A: Source>(
        mut self,
        sources: impl IntoIterator<Item = A>,
    ) -> AnySource<impl Source> {
        AnySource {
            sources: EitherSource(ManySources(Vec::from_iter(sources)), self.sources),
        }
    }
}

impl<S> FromIterator<S> for AnySource<ManySources<S>> {
    fn from_iter<I: IntoIterator<Item = S>>(other: I) -> Self {
        AnySource {
            sources: ManySources(Vec::from_iter(other)),
        }
    }
}

impl<S: Source> Source for AnySource<S> {
    fn modified(&self, path: &str) -> Result<u64, Error> {
        self.sources.modified(path)
    }

    fn load(&self, path: &str) -> Result<Vec<u8>, Error> {
        self.sources.load(path)
    }

    fn load_with_metadata(&self, path: &str) -> Result<(Vec<u8>, u64), Error> {
        self.sources.load_with_metadata(path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
