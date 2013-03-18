#!/usr/bin/env python

import glob
import codecs
import os

if not os.path.exists('./tmp'):
    os.makedirs('./tmp')

for filename in glob.glob('./src/*.r*'):
    output = filename.replace('/src/', '/tmp/')
    with codecs.open(filename, 'r', 'utf8') as file:
        data = file.read().encode('unicode-escape').replace('\\u', '\\\\u').decode('unicode-escape')
        with codecs.open(output, 'w', 'utf8') as out:
            out.write(data)
