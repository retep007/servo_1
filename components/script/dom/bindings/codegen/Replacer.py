#!/usr/bin/python

import os, fnmatch, sys
def findReplace(directory, filePattern, glbl):
    for path, dirs, files in os.walk(os.path.abspath(directory)):
        for filename in fnmatch.filter(files, filePattern):
            filepath = os.path.join(path, filename)
            with open(filepath) as f:
                s = f.read()
            for generic in glbl['GenericStructs']:
            	for rep in replacing:
            		s = s.replace(rep[0] % generic, rep[1] % generic)
            with open(filepath, "w") as f:
                f.write(s)

if __name__ == '__main__':
    glbl = {}
    execfile(os.path.normpath(sys.argv[1]), glbl)
    path = os.path.normpath(sys.argv[2])
    findReplace(path, "*.rs", glbl)
      
replacing = [
        ("&%(generic)s," , "&%(generic)s<TH>," ),
        ("&%(generic)s)" , "&%(generic)s<TH>)" ),
        ("<&%(generic)s>" , "<&%(generic)s<TH>>" ),
        ("<%(generic)s>" , "<%(generic)s<TH>>" ),
        ("-> %(generic)s {" , "-> %(generic)s<TH> {" ),
        ("&'a %(generic)s)" , "&'a %(generic)s<TH>)" ),
        ("&'a %(generic)s," , "&'a %(generic)s<TH>," ),
        ("<&'a %(generic)s>" , "<&'a %(generic)s<TH>>" ),
        ("-> &%(generic)s {" , "-> &%(generic)s<TH> {" ),
        (": %(generic)s)" , ": %(generic)s<TH>)" ),
        (": %(generic)s," , ": %(generic)s<TH>," ),

]