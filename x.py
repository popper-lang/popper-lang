import os
import sys

def _is_exe(fpath):
    return os.path.isfile(fpath) and os.access(fpath, os.X_OK)

def _which(program):
    fpath, fname = os.path.split(program)
    if fpath:
        if _is_exe(program):
            return program
    else:
        for path in os.environ.get("PATH", "").split(os.pathsep):
            exe_file = os.path.join(path, program)
            if _is_exe(exe_file):
                return exe_file

    return None

if __name__ == '__main__':
	