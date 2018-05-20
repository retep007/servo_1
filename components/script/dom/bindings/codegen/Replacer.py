#!/usr/bin/python

import os, fnmatch, sys

      
replacing = [
        ("&%(generic)s," , "&%(generic)s<TH>," ),
        ("&%(generic)s)" , "&%(generic)s<TH>)" ),
        ("<&%(generic)s>" , "<&%(generic)s<TH>>" ),
        ("<%(generic)s>" , "<%(generic)s<TH>>" ),
        ("-> %(generic)s {" , "-> %(generic)s<TH> {" ),
        ("-> %(generic)s;" , "-> %(generic)s<TH>;" ),
        ("&'a %(generic)s)" , "&'a %(generic)s<TH>)" ),
        ("&'a %(generic)s," , "&'a %(generic)s<TH>," ),
        ("<&'a %(generic)s>" , "<&'a %(generic)s<TH>>" ),
        ("-> &%(generic)s {" , "-> &%(generic)s<TH> {" ),
        ("-> &'a %(generic)s {" , "-> &'a %(generic)s<TH> {" ),
        (": %(generic)s)" , ": %(generic)s<TH>)" ),
        (": %(generic)s," , ": %(generic)s<TH>," ),
        (": &[&%(generic)s]" , ": &[&%(generic)s<TH>]" ),
        ("type Extension = %(generic)s;", "type Extension = %(generic)s<TH>;"),
        ("type Item = %(generic)s;", "type Item = %(generic)s<TH>;"),
        ("type Value = %(generic)s;", "type Value = %(generic)s<TH>;"),
        ("type Status = %(generic)s;", "type Status = %(generic)s<TH>;"),
]

def findReplace(directory, filePattern, glbl):
    for path, dirs, files in os.walk(os.path.abspath(directory)):
        for filename in fnmatch.filter(files, filePattern):
            filepath = os.path.join(path, filename)
            print filename
            with open(filepath) as f:
                s = f.read()
            for generic in glbl['GenericStructs']:
            	for rep in replacing:
            		s = s.replace(rep[0] % {'generic' : generic}, rep[1] % {'generic': generic})
            with open(filepath, "w") as f:
                f.write(s)

if __name__ == '__main__':
    glbl = {}
    execfile(os.path.normpath(sys.argv[1]), glbl)
    path = os.path.normpath(sys.argv[2])
    findReplace(path, "*.rs", glbl)
