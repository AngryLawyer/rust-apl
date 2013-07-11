#!/usr/bin/env python

import codecs
import os
import fnmatch

if not os.path.exists('./tmp'):
    os.makedirs('./tmp')

for root, dirnames, filenames in os.walk('./src'):
    for dirname in dirnames:
        dirname = os.path.join(root, dirname)
        output = dirname.replace('/src/', '/tmp/')
        if not os.path.exists(output):
            os.makedirs(output)

    for filename in fnmatch.filter(filenames, '*.r*'):
        if filename[0] == '.':
            continue

        filename = os.path.join(root, filename)
        output = filename.replace('/src/', '/tmp/')
        with codecs.open(filename, 'r', 'utf8') as file:
            print filename
            data = file.read().encode('unicode-escape').replace('\\u', '\\\\u').decode('unicode-escape')
            with codecs.open(output, 'w', 'utf8') as out:
                out.write(data)
