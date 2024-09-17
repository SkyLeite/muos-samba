use std::{
    cmp::Ordering,
    collections::VecDeque,
    path::{Path, PathBuf},
};

use crate::prelude::*;
use egui::Label;
use fs_tree::{iter::Iter, FsTree};

pub struct FileTree {
    tree: FsTree,
}

impl FileTree {
    pub fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let fs_tree = FsTree::symlink_read_at(path)?;
        Ok(Self { tree: fs_tree })
    }

    pub fn from_tree(tree: FsTree) -> Self {
        Self { tree }
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("/")
            .default_open(false)
            .show(ui, |other_ui| {
                other_ui.label("Test");
                FileTree::render_tree(&PathBuf::from("/"), &self.tree, other_ui);
            });
    }

    pub fn render_tree(path: &PathBuf, tree: &FsTree, ui: &mut egui::Ui) {
        // Fetch children of the current node, if they exist
        if path.file_name().unwrap_or_default().to_string_lossy() == ".cargo" {
            dbg!(path);
            dbg!(tree.children());

            if let Some(children) = tree.children() {
                for (path, subtree) in children.iter() {
                    dbg!(path);
                    dbg!(path.is_dir());
                }
            }
        }

        if path.file_name().unwrap_or_default().to_string_lossy() == "config.toml" {
            dbg!(path);
            dbg!(tree.children());

            if let Some(children) = tree.children() {
                for (path, subtree) in children.iter() {
                    dbg!(path);
                    dbg!(path.is_dir());
                }
            }

            ui.label("HIIII XD");
            ui.label(path.file_name().unwrap_or_default().to_string_lossy());
        }

        if !path.is_dir() {
            ui.label(path.file_name().unwrap_or_default().to_string_lossy());
        }

        if let Some(children) = tree.children() {
            for (path, subtree) in children.iter() {
                if path.is_file() {
                    // Render a label for files
                    ui.label(path.file_name().unwrap_or_default().to_string_lossy());
                } else if path.is_dir() {
                    // Render a directory as a CollapsingHeader
                    egui::CollapsingHeader::new(
                        path.file_name().unwrap_or_default().to_string_lossy(),
                    )
                    .id_source(path.to_string_lossy())
                    .show(ui, |ui| {
                        FileTree::render_tree(path, subtree, ui);
                    });
                }
            }
        }
    }

    // pub fn render_tree(tree: &FsTree, ui: &mut egui::Ui) {
    //     for (subtree, path) in FileTree::breadth_first_iterator(tree) {
    //         if path.is_file() {
    //             ui.label(path.file_name().unwrap_or_default().to_string_lossy());
    //         } else if path.is_dir() {
    //             ui.collapsing(
    //                 path.file_name().unwrap_or_default().to_string_lossy(),
    //                 |ui| {
    //                     FileTree::render_tree(subtree, ui);
    //                 },
    //             );
    //         }
    //     }
    // }

    pub fn breadth_first_iterator<'a>(
        tree: &'a FsTree,
    ) -> impl Iterator<Item = (&'a FsTree, PathBuf)> {
        let mut queue = VecDeque::new();

        // Start with the root
        queue.push_back((tree, PathBuf::new()));

        std::iter::from_fn(move || {
            if let Some((current_tree, current_path)) = queue.pop_front() {
                // Add all the children of the current node to the queue
                for (subtree, subpath) in current_tree.iter() {
                    let mut new_path = current_path.clone();
                    new_path.push(subpath.file_name().unwrap_or_default());
                    queue.push_back((subtree, new_path));
                }
                Some((current_tree, current_path))
            } else {
                // If the queue is empty, we're done
                None
            }
        })
    }
}

// let iter = tree.iter();
// let mut vec: Vec<(&FsTree, PathBuf)> = iter.collect();
// vec.sort_by(|_, (_, path_b)| {
//     if path_b.is_dir() {
//         return Ordering::Greater;
//     } else {
//         return Ordering::Less;
//     }
// });
