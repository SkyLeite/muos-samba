use crate::file_tree::FileTree;

trait FileSystem {
    fn get_tree() -> FileTree;
}
