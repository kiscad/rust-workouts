import shutil
import os
from pathlib import Path

SRC_DIR = 'D:/Documents/projects/rust/'
DST_DIR = 'D:/Documents/projects/rust-symlinks/'
FILE_EXT = ["rs", "c", "cc", "cpp", "cxx", "h", "hpp", "hxx", "java", "py", "kt", "js", "lua"]


def ignore_files(dir, files):
    return [f for f in files if os.path.isfile(os.path.join(dir, f))]

def file_gen(rootdir: str):
    count = 0
    file_types = tuple("**/*." + ext for ext in FILE_EXT)
    for ftype in file_types:
        for filepath in Path(rootdir).glob(ftype):
            yield filepath

def create_symlinks(srcdir: str):
    for fpath in file_gen(srcdir):
        linkpath = str(fpath).replace("rust", "rust-symlinks", 1)
        try:
            Path(linkpath).symlink_to(fpath)
        except:
            print("=", end='')
        # print(linkpath)
        # print(fpath)
        # break
    print()

def copy_files(srcdir: str):
    for fpath in file_gen(srcdir):
        copypath = str(fpath).replace("rust", "rust-symlinks", 1)
        try:
            shutil.copy(fpath, copypath)
        except:
            print("=", end='')
    print()

shutil.copytree(SRC_DIR, DST_DIR, ignore=ignore_files)
# create_symlinks(SRC_DIR)
copy_files(SRC_DIR)


