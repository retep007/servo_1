#!/usr/bin/python

import os, fnmatch, sys
def findReplace(directory, find, replace, filePattern):
    for path, dirs, files in os.walk(os.path.abspath(directory)):
        for filename in fnmatch.filter(files, filePattern):
            filepath = os.path.join(path, filename)
            with open(filepath) as f:
                s = f.read()
            s = s.replace(find, replace)
            with open(filepath, "w") as f:
                f.write(s)

if __name__ == '__main__':
    glbl = {}
    execfile(os.path.normpath(sys.argv[1]), glbl)
    for generic in glbl['GenericStructs']:
        replaces = {}
        path = os.path.normpath(sys.argv[2])
        replaces['generic'] = generic
        findReplace(path, "&%(generic)s," % replaces, "&%(generic)s<TH>," % replaces, "*.rs");
        findReplace(path, "&%(generic)s)" % replaces, "&%(generic)s<TH>)" % replaces, "*.rs");
        findReplace(path, "<&%(generic)s>" % replaces, "<&%(generic)s<TH>>" % replaces, "*.rs");
        findReplace(path, "<%(generic)s>" % replaces, "<%(generic)s<TH>>" % replaces, "*.rs");
        findReplace(path, "-> %(generic)s {" % replaces, "-> %(generic)s<TH> {" % replaces, "*.rs");
        findReplace(path, "&'a %(generic)s)" % replaces, "&'a %(generic)s)" % replaces, "*.rs");
        findReplace(path, "&'a %(generic)s," % replaces, "&'a %(generic)s)" % replaces, "*.rs");
        findReplace(path, "&'a %(generic)s," % replaces, "&'a %(generic)s," % replaces, "*.rs");
        findReplace(path, "<&'a %(generic)s>" % replaces, "<&'a %(generic)s<TH>>" % replaces, "*.rs");
        findReplace(path, "-> &%(generic)s {" % replaces, "-> &%(generic)s<TH> {" % replaces, "*.rs");
