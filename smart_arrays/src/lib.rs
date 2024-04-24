use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

struct UniqueStr {
    data: Box<[u8]>,
}

impl UniqueStr {
    fn new(data: &str) -> Self {
        Self {
            data: Box::from(data.as_bytes()),
        }
    }
}

impl Clone for UniqueStr {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl Deref for UniqueStr {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}

impl DerefMut for UniqueStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.data
    }
}

struct CowStr {
    data: Rc<[u8]>,
}

impl CowStr {
    fn new(data: &str) -> Self {
        Self {
            data: Rc::from(data.as_bytes()),
        }
    }
}

impl Clone for CowStr {
    fn clone(&self) -> Self {
        Self {
            data: Rc::clone(&self.data),
        }
    }
}

impl Deref for CowStr {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}

impl DerefMut for CowStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if Rc::strong_count(&self.data) != 1 {
            self.data = Rc::from(&*self.data);
        }
        Rc::get_mut(&mut self.data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_not_shared() {
        let s1 = UniqueStr::new("hello unique");
        let mut s2 = s1.clone();
        assert_ne!(s1.data.as_ptr(), s2.data.as_ptr());
        s2[0] = b'b';
        assert_ne!(s1.data.as_ptr(), s2.data.as_ptr());
        assert_eq!(&*s1.data, "hello unique".as_bytes());
        assert_eq!(&*s2.data, "bello unique".as_bytes());
    }

    #[test]
    fn cow_shared() {
        let s1 = CowStr::new("hello shared");
        let mut s2 = s1.clone();
        assert_eq!(s1.data.as_ptr(), s2.data.as_ptr());
        s2[0] = b'b';
        assert_ne!(s1.data.as_ptr(), s2.data.as_ptr());
        assert_eq!(&*s1.data, "hello shared".as_bytes());
        assert_eq!(&*s2.data, "bello shared".as_bytes());
    }
}
