#! /usr/bin/env python3

import scratchattach as scratch3
import argparse
import os
import colorama

colorama.init()

parser = argparse.ArgumentParser(prog='scratch2exe')
parser.add_argument('-c', action='store_true')
parser.add_argument('--to_c', action='store_true', help='Compile to C instead of C++')
parser.add_argument('-o', nargs='?', default='output', help='Output file')
parser.add_argument('--json', nargs='?', default='project.json', help='json output file')
parser.add_argument('--scratchnative', nargs='?', default='scratchnative', help='path to scratchnative executable')
parser.add_argument('action', nargs='+')
parser.add_argument('project_id', nargs='+')

args = parser.parse_args()

project = scratch3.get_project(int(args.project_id[0]))

def fetch_project():
    print("=> Downloading project...", end=" ");
    project.download(filename=args.json, dir='');
    print(colorama.Fore.GREEN + "Done" + colorama.Style.RESET_ALL)


def transpile_project():
    print("=> Compiling Scratch project...", end=" ");
    os.system(f'{args.scratchnative} {args.json}.sb3 -o {args.o if args.c else "output.cpp"}')
    print(colorama.Fore.GREEN + "Done" + colorama.Style.RESET_ALL)

def compile_project():
    print(f"=> Compiling {'C++' if not args.to_c else 'C'} output...", end=" ");
    os.system(f'{"c++" if not args.to_c else "cc"} output.cpp -o {args.o}');
    print(colorama.Fore.GREEN + "Done" + colorama.Style.RESET_ALL)

if args.action[0] == 'fetch':
    fetch_project();
    print("Fetched project!")

elif args.action[0] == 'build':
    fetch_project()
    transpile_project()

    if args.c == False:
        compile_project()
        os.remove('output.cpp')

    os.remove(f'{args.json}.sb3')

    print("Project Built!");
