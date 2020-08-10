use super::Tag;

pub trait Index {
    fn index_into<'a>(&self, tag: &'a Tag) -> Option<&'a Tag>;
    fn index_into_mut<'a>(&self, tag: &'a mut Tag) -> Option<&'a mut Tag>;
    fn index_or_insert<'a>(&self, tag: &'a mut Tag) -> &'a mut Tag;
}

impl Index for str {
    fn index_into<'a>(&self, tag: &'a Tag) -> Option<&'a Tag> {
        match tag {
            Tag::Compound(m) => m.get(self),
            _ => None,
        }
    }

    fn index_into_mut<'a>(&self, tag: &'a mut Tag) -> Option<&'a mut Tag> {
        match tag {
            Tag::Compound(m) => m.get_mut(self),
            _ => None,
        }
    }

    fn index_or_insert<'a>(&self, tag: &'a mut Tag) -> &'a mut Tag {
        match tag {
            Tag::Compound(m) => {
                let key = self.to_owned();

                m.entry(key.clone())
                    .or_insert(Tag::End)
            },
            _ => panic!("cannot index tag type: {:?}", tag.kind()),
        }
    }
}

impl Index for String {
    fn index_into<'a>(&self, tag: &'a Tag) -> Option<&'a Tag> {
        self[..].index_into(tag)
    }

    fn index_into_mut<'a>(&self, tag: &'a mut Tag) -> Option<&'a mut Tag> {
        self[..].index_into_mut(tag)
    }

    fn index_or_insert<'a>(&self, tag: &'a mut Tag) -> &'a mut Tag {
        self[..].index_or_insert(tag)
    }
}

impl Index for usize {
    fn index_into<'a>(&self, tag: &'a Tag) -> Option<&'a Tag> {
        match tag {
            Tag::List(_, v) => v.get(*self),
            _ => None,
        }
    }

    fn index_into_mut<'a>(&self, tag: &'a mut Tag) -> Option<&'a mut Tag> {
        match tag {
            Tag::List(_, v) => v.get_mut(*self),
            _ => None,
        }
    }

    fn index_or_insert<'a>(&self, tag: &'a mut Tag) -> &'a mut Tag {
        match tag {
            Tag::List(_, v) => {
                let len = v.len();

                v.get_mut(*self)
                    .unwrap_or_else(|| panic!(
                        "cannot access index {} in array of length {}",
                        self,
                        len,
                    ))
            },
            _ => panic!("cannot access index of tag type: {:?}", tag.kind()),
        }
    }
}

impl<'b, T: ?Sized + Index> Index for &'b T {
    fn index_into<'a>(&self, tag: &'a Tag) -> Option<&'a Tag> {
        (**self).index_into(tag)
    }

    fn index_into_mut<'a>(&self, tag: &'a mut Tag) -> Option<&'a mut Tag> {
        (**self).index_into_mut(tag)
    }

    fn index_or_insert<'a>(&self, tag: &'a mut Tag) -> &'a mut Tag {
        (**self).index_or_insert(tag)
    }
}
