use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

struct UniqueStr {
    data: Box<[u8]>,
}

impl UniqueStr {
    fn new(data: &str) -> Self {
    }
}

impl Clone for UniqueStr {
    fn clone(&self) -> Self {
    }
}

impl Deref for UniqueStr {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
    }
}

impl DerefMut for UniqueStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
    }
}

struct CowStr {
    data: Rc<[u8]>,
}

impl CowStr {
    fn new(data: &str) -> Self {
        Self {
        }
    }
}

impl Clone for CowStr {
    fn clone(&self) -> Self {
        Self {
        }
    }
}

impl Deref for CowStr {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
    }
}

impl DerefMut for CowStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_not_shared() {
    }

    #[test]
    fn cow_shared() {
    }
}
