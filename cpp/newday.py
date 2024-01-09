import os
import sys
import shutil

newday = sys.argv[1]
templatefn = "template.cpp"
path = os.path.join(newday, newday)
newfilepath = path + ".cpp"
content = ""

print(path)

if not os.path.isdir(newday):
    os.makedirs(newday)
    shutil.copyfile("./" + templatefn, newfilepath)

    with open(newfilepath, 'r+') as file:
        content = file.read()
        content = content.replace("0x-test.txt", newday + "-test.txt")
        content = content.replace("0x.txt", newday + ".txt")
        file.seek(0)
        file.write(content)
        file.truncate()

    testinput = open(path + "-test.txt", 'x')
    offinput  = open(path + ".txt", 'x')

else:
    print(f"You already started the {newday} day!")