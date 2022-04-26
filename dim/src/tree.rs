use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Entry<T> {
    Directory {
        folder: String,
        files: Vec<Entry<T>>,
    },
    File(T),
}

impl<T> Entry<T> {
    pub fn new() -> Self {
        Self::Directory {
            folder: "/".into(),
            files: vec![],
        }
    }

    pub fn build_with<U>(
        values: impl IntoIterator<Item = U>,
        k: impl Fn(&U) -> Vec<String>,
        v: impl Fn(&str, U) -> T,
    ) -> Self {
        let mut entry = Self::new();

        for value in values.into_iter() {
            let mut components = k(&value);

            let filename = match components.pop() {
                Some(x) => x,
                None => continue,
            };

            entry.insert(components.iter(), v(filename.as_ref(), value));
        }

        // remove root-directories with only one folder inside.
        entry.compress();

        entry
    }

    pub fn insert<'a>(&mut self, mut keys: impl Iterator<Item = impl AsRef<str>>, value: T) {
        if self.is_file() {
            return;
        }

        // NOTE: If the key is None, we've reached the max depth, so we can safely insert the
        // record into our current self.
        let key = match keys.next() {
            Some(x) => x,
            None => {
                self.insert_unchecked(Self::File(value));
                return;
            }
        };

        if key.as_ref() == "/" {
            self.insert(keys, value);
            return;
        }

        // If we get a key, we want to create a new entry in self for this key, or access it and
        // insert `value` recursively.
        let files = self.files_unchecked();
        let folder = if let Some(x) = files.iter_mut().find(|x| x.is_dir(key.as_ref())) {
            x
        } else {
            files.push(Self::Directory {
                folder: key.as_ref().to_owned(),
                files: vec![],
            });
            // SAFETY: Last element will always exist, because we push it above.
            match files.last_mut() {
                Some(x) => x,
                None => unreachable!("Entry::insert: failed to get last file"),
            }
        };

        folder.insert(keys, value);
    }

    /// Changes the root to the first folder that has children.
    pub fn compress(&mut self) {
        while let Entry::Directory { files, .. } = self {
            if files.len() != 1 {
                return;
            }

            if let Some(Entry::File(_)) = files.first() {
                return;
            }

            let mut new_root = files.pop().unwrap();

            core::mem::swap(self, &mut new_root);
        }
    }

    fn insert_unchecked(&mut self, value: Self) {
        match self {
            Self::Directory { files, .. } => files.push(value),
            _ => unreachable!("Entry::insert"),
        }
    }

    fn files_unchecked(&mut self) -> &mut Vec<Self> {
        match self {
            Self::Directory { files, .. } => files,
            _ => unreachable!("Entry::files_unchecked"),
        }
    }

    fn is_dir(&self, k: &str) -> bool {
        match self {
            Self::File(_) => false,
            Self::Directory { folder, .. } => folder == k,
        }
    }

    fn is_file(&self) -> bool {
        match self {
            Self::File(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Record(String);

    #[test]
    fn insert_some() {
        let mut entry = Entry::new();

        // file located at /a/b/c
        let keys = ["a", "b", "c"];

        entry.insert(IntoIterator::into_iter(keys), Record("d.txt".to_string()));
        entry.insert(IntoIterator::into_iter(keys), Record("d.txt".to_string()));

        assert_eq!(
            entry,
            Entry::Directory {
                folder: "/".into(),
                files: vec![Entry::Directory {
                    folder: "a".into(),
                    files: vec![Entry::Directory {
                        folder: "b".into(),
                        files: vec![Entry::Directory {
                            folder: "c".into(),
                            files: vec![
                                Entry::File(Record("d.txt".to_string())),
                                Entry::File(Record("d.txt".to_string()))
                            ]
                        }]
                    }]
                }]
            }
        );
    }

    #[test]
    fn insert_different_folders() {
        let mut entry = Entry::new();

        entry.insert(
            IntoIterator::into_iter(["a", "b", "c"]),
            Record("d.txt".to_string()),
        );
        entry.insert(
            IntoIterator::into_iter(["a", "b", "d"]),
            Record("d.txt".to_string()),
        );
        entry.insert(
            IntoIterator::into_iter(["z", "b", "d"]),
            Record("d.txt".to_string()),
        );
        entry.insert(
            IntoIterator::into_iter(["a", "z"]),
            Record("d.txt".to_string()),
        );

        assert_eq!(
            entry,
            Entry::Directory {
                folder: "/".into(),
                files: vec![
                    Entry::Directory {
                        folder: "a".into(),
                        files: vec![
                            Entry::Directory {
                                folder: "b".into(),
                                files: vec![
                                    Entry::Directory {
                                        folder: "c".into(),
                                        files: vec![Entry::File(Record("d.txt".to_string())),]
                                    },
                                    Entry::Directory {
                                        folder: "d".into(),
                                        files: vec![Entry::File(Record("d.txt".to_string())),]
                                    }
                                ]
                            },
                            Entry::Directory {
                                folder: "z".into(),
                                files: vec![Entry::File(Record("d.txt".to_string())),]
                            }
                        ]
                    },
                    Entry::Directory {
                        folder: "z".into(),
                        files: vec![Entry::Directory {
                            folder: "b".into(),
                            files: vec![Entry::Directory {
                                folder: "d".into(),
                                files: vec![Entry::File(Record("d.txt".to_string())),]
                            }]
                        }]
                    },
                ]
            }
        );
    }

    #[test]
    fn test_compression() {
        let mut entry = Entry::new();

        entry.insert(
            IntoIterator::into_iter(["a", "b", "c"]),
            Record("d.txt".into()),
        );
        entry.compress();

        assert_eq!(
            entry,
            Entry::Directory {
                folder: "c".into(),
                files: vec![Entry::File(Record("d.txt".into()))]
            }
        );
    }
}
