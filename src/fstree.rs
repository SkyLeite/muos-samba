use im::OrdMap;
use std::path::{Path, PathBuf};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FsTree {
    children: OrdMap<PathBuf, FsTree>,
}

impl FsTree {
    // Create an empty FsTree
    pub fn new() -> Self {
        FsTree {
            children: OrdMap::new(),
        }
    }

    // Insert a new file or directory into the tree
    pub fn insert(&mut self, path: PathBuf, subtree: FsTree) {
        self.children.insert(path, subtree);
    }

    // Build a FsTree from a directory path
    pub fn from_path(path: &Path) -> Self {
        let mut tree = FsTree::new();
        Self::build_tree(&path, &mut tree, 10, 0);
        tree
    }

    fn build_tree(path: &Path, tree: &mut FsTree, max_depth: usize, current_depth: usize) {
        if path.is_dir() {
            for entry in std::fs::read_dir(path).unwrap() {
                if let Ok(entry) = entry {
                    let child_path = entry.path();
                    let mut child_tree = FsTree::new();
                    if current_depth <= max_depth {
                        Self::build_tree(
                            &child_path,
                            &mut child_tree,
                            max_depth,
                            current_depth + 1,
                        );
                    }

                    tree.insert(child_path.into(), child_tree);
                }
            }
        }
    }

    pub fn render_tree(ui: &mut egui::Ui, tree: &FsTree) {
        // Separate directories and files
        let mut directories: Vec<(PathBuf, &FsTree)> = Vec::new();
        let mut files: Vec<(PathBuf, &FsTree)> = Vec::new();

        for (path, subtree) in &tree.children {
            if path.is_dir() {
                directories.push((path.clone(), subtree));
            } else if path.is_file() {
                files.push((path.clone(), subtree));
            }
        }

        // Render directories first
        for (dir_path, subtree) in directories {
            let response = ui.collapsing(
                dir_path.file_name().unwrap_or_default().to_string_lossy(),
                |ui| {
                    FsTree::render_tree(ui, subtree);
                },
            );

            if response.header_response.gained_focus() {
                response.header_response.scroll_to_me(None);
            }
        }

        // Render files
        for (file_path, _) in files {
            let name = file_path.file_name().unwrap_or_default().to_string_lossy();
            let response = ui.add(egui::Label::new(name).sense(egui::Sense::click()));

            if response.gained_focus() {
                response.scroll_to_me(None);
            }
        }
    }
}
