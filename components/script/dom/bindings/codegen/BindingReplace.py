#!/usr/bin/python

import os, fnmatch, sys, re

      
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

def replace(s):
    s = re.sub('const ([a-zA-Z_0-9]+): JSJitInfo = JSJitInfo {', 'fn \\1<TH: TypeHolderTrait>() -> JSJitInfo { JSJitInfo {', s)
    s = re.sub('call: ([a-zA-Z_0-9]+) as \*const', 'call: \\1::<TH> as *const', s)
    s = re.sub('\),\n};', '),\n}}', s);
    s = re.sub('info: &([a-zA-Z_0-9]+) ', 'info: &\\1::<TH>() ', s);
    s = re.sub('const sMethods_specs: &\'static \[&\'static\[JSFunctionSpec\]\] = &\[', 'fn sMethods_specs<TH: TypeHolderTrait>() -> &\'static [&\'static[JSFunctionSpec]] { &[', s);
    s = re.sub('\]\n\n\];', ']\n\n]}', s);
    s = re.sub('const sMethods: &\'static \[Guard<&\'static \[JSFunctionSpec\]>\] = &\[', 'fn sMethods<TH: TypeHolderTrait>() -> &\'static [Guard<&\'static [JSFunctionSpec]>] { &[', s);
    s = re.sub('sMethods_specs\[', 'sMethods_specs::<TH>()[', s);
    s = re.sub('const sAttributes_specs: &\'static \[&\'static\[JSPropertySpec\]\] = &\[', 'fn sAttributes_specs<TH: TypeHolderTrait>() -> &\'static [&\'static[JSPropertySpec]] { &[', s);
    s = re.sub('const sAttributes: &\'static \[Guard<&\'static \[JSPropertySpec\]>\] = &\[', 'fn sAttributes<TH: TypeHolderTrait>() -> &\'static [Guard<&\'static [JSPropertySpec]>] { &[', s);
    s = re.sub('sAttributes_specs\[', 'sAttributes_specs::<TH>()[', s);
    s = re.sub('fn sAttributes<TH: TypeHolderTrait>\(\) -> &\'static \[Guard<&\'static \[JSPropertySpec\]>] { &\[\n(.+)\n\];', 'fn sAttributes<TH: TypeHolderTrait>() -> &\'static [Guard<&\'static [JSPropertySpec]>] { &[\n\\1\n]}', s);
    s = re.sub('fn sMethods<TH: TypeHolderTrait>\(\) -> &\'static \[Guard<&\'static \[JSFunctionSpec\]>\] { &\[\n(.+)\n\];', 'fn sMethods<TH: TypeHolderTrait>() -> &\'static [Guard<&\'static [JSFunctionSpec]>] { &[\n\\1\n]}', s);
    s = re.sub(' sMethods,', ' sMethods::<TH>(),', s);
    s = re.sub(' sAttributes,', ' sAttributes::<TH>(),', s);

    s = re.sub('const sUnforgeableAttributes_specs: &\'static \[&\'static\[JSPropertySpec\]\] = &\[', 'fn sUnforgeableAttributes_specs<TH: TypeHolderTrait>() -> &\'static [&\'static[JSPropertySpec]] { &[', s);
    s = re.sub('const sUnforgeableAttributes: &\'static \[Guard<&\'static \[JSPropertySpec\]>\] = &\[', 'fn sUnforgeableAttributes<TH: TypeHolderTrait>() -> &\'static [Guard<&\'static [JSPropertySpec]>] { &[', s);
    s = re.sub('fn sUnforgeableAttributes<TH: TypeHolderTrait>\(\) -> &\'static \[Guard<&\'static \[JSPropertySpec\]>] { &\[\n(.+)\n\];', 'fn sUnforgeableAttributes<TH: TypeHolderTrait>() -> &\'static [Guard<&\'static [JSPropertySpec]>] { &[\n\\1\n]}', s);
    s = re.sub('sUnforgeableAttributes_specs\[', 'sUnforgeableAttributes_specs::<TH>()[', s);
    s = re.sub(' sUnforgeableAttributes,', ' sUnforgeableAttributes::<TH>(),', s);

    s = re.sub('const sUnforgeableMethods_specs: &\'static \[&\'static\[JSFunctionSpec\]\] = &\[', 'fn sUnforgeableMethods_specs<TH: TypeHolderTrait>() -> &\'static [&\'static[JSFunctionSpec]] { &[', s);
    s = re.sub('const sUnforgeableMethods: &\'static \[Guard<&\'static \[JSFunctionSpec\]>\] = &\[', 'fn sUnforgeableMethods<TH: TypeHolderTrait>() -> &\'static [Guard<&\'static [JSFunctionSpec]>] { &[', s);
    s = re.sub('fn sUnforgeableMethods<TH: TypeHolderTrait>\(\) -> &\'static \[Guard<&\'static \[JSFunctionSpec\]>] { &\[\n(.+)\n\];', 'fn sUnforgeableMethods<TH: TypeHolderTrait>() -> &\'static [Guard<&\'static [JSFunctionSpec]>] { &[\n\\1\n]}', s);
    s = re.sub('sUnforgeableMethods_specs\[', 'sUnforgeableMethods_specs::<TH>()[', s);
    s = re.sub(' sUnforgeableMethods,', ' sUnforgeableMethods::<TH>(),', s);
   
    s = re.sub('const sStaticMethods_specs: &\'static \[&\'static\[JSFunctionSpec\]\] = &\[', 'fn sStaticMethods_specs<TH: TypeHolderTrait>() -> &\'static [&\'static[JSFunctionSpec]] { &[', s);
    s = re.sub('const sStaticMethods: &\'static \[Guard<&\'static \[JSFunctionSpec\]>\] = &\[', 'fn sStaticMethods<TH: TypeHolderTrait>() -> &\'static [Guard<&\'static [JSFunctionSpec]>] { &[', s);
    s = re.sub('fn sStaticMethods<TH: TypeHolderTrait>\(\) -> &\'static \[Guard<&\'static \[JSFunctionSpec\]>] { &\[\n(.+)\n\];', 'fn sStaticMethods<TH: TypeHolderTrait>() -> &\'static [Guard<&\'static [JSFunctionSpec]>] { &[\n\\1\n]}', s);
    s = re.sub('sStaticMethods_specs\[', 'sStaticMethods_specs::<TH>()[', s);
    s = re.sub(' sStaticMethods,', ' sStaticMethods::<TH>(),', s);
    s = re.sub(' op: Some\((?!generic)([a-zA-Z_]+)\),', ' op: Some(\\1::<TH>),',s);

    s = re.sub('}\nconst sConstants:', ';\nconst sConstants:', s);
    s = re.sub('const ([a-zA-Z_0-9]+): JSTypedMethodJitInfo = JSTypedMethodJitInfo {', 'fn \\1<TH: TypeHolderTrait>() -> JSTypedMethodJitInfo { JSTypedMethodJitInfo {', s)
    s = re.sub('JSJitInfo_ArgType,\n};', 'JSJitInfo_ArgType,\n}}', s)
    if 'call(_constructor)' in s:
        s = re.sub('call\(_constructor\)', 'call(_constructor::<TH>)',s)
        s = re.sub('static ([a-zA-Z_]+): ([a-zA-Z]+) =([\w\s:()&<>,\"{}\\\[\]]+);', 'fn \\1<TH: TypeHolderTrait>() -> \\2 {\n \\3 \n}',s)
        s = re.sub('&INTERFACE_OBJECT_CLASS,', '&INTERFACE_OBJECT_CLASS::<TH>(),',s)

    return s
def findReplace(directory, filePattern):
    for path, dirs, files in os.walk(os.path.abspath(directory)):
        for filename in fnmatch.filter(files, filePattern):
            filepath = os.path.join(path, filename)
            print filename
            with open(filepath) as f:
                s = f.read()
            if 'impl<TH: TypeHolderTrait> PartialEq for ' in s:
            	s = replace(s)
            with open(filepath, "w") as f:
                f.write(s)

if __name__ == '__main__':
    path = os.path.normpath(sys.argv[1])
    findReplace(path, "*Binding.rs")
