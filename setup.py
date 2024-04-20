#!/usr/bin/env python3

# use urllib because it's the default
import urllib.request
import ssl
import json
import platform
import sys
import os
import tempfile
import tarfile
import shutil



def install_release():

	print("!!!!!!! Running actual installer for github release")
	# because python default library sucks and can't load certs for sh*t without requests module
	# but we dont want to install external dependencies now
	context = ssl._create_unverified_context()

	print("getting latest release..")
	f = urllib.request.urlopen('https://api.github.com/repos/h4sh5/rcut/releases/latest',context=context)

	d = json.loads(f.read().decode('utf-8'))
	print("found release:", d['name'])

	# choose release based on platform and architecture
	arch = platform.machine()

	system = sys.platform

	for a in d['assets']:
		name = a['name']
		if system in name and arch in name or (system == 'darwin' and system in name):
			# this is a good release

			print("found matching release asset for system:", name)
			download_url = a['browser_download_url']
			print("downloading from %s ..."%download_url)
			f = urllib.request.urlopen(download_url, context=context)
			# with tempfile.mkdtemp() as tmpdirname:
			tmpdirname = tempfile.mkdtemp()
			with open(os.path.join(tmpdirname, name),'wb') as file:
				file.write(f.read())
				print("written to %s" % os.path.join(tmpdirname, name))
			# extract
			if name.endswith('.tar.gz'): # this is for unix systems
				tar = tarfile.open(os.path.join(tmpdirname, name),"r:gz")
				tar.extractall(path=tmpdirname)
				tar.close()

				extracted_binary = os.path.join(tmpdirname, name.split(".tar.gz")[0], "rcut")
				print('extracted binary to %s' % extracted_binary)


			if name.endswith(".zip"): # for windows
				with zipfile.ZipFile(os.path.join(tmpdirname, name), 'r') as zipf:
					 zipf.extractall(tmpdirname)
				extracted_binary = os.path.join(tmpdirname, name.split(".zip")[0], "rcut.exe")
				print('extracted binary to %s' % extracted_binary)

			# add to $PATH
			print("attempting to add to $PATH..")
			print("PATH:", os.getenv("PATH"))

			if os.getenv("PATH") == None:
				print("no PATH variable found, exiting..")
				exit(1)

			for path_dir in os.getenv("PATH").split(":")[::-1]: # do it backwards for most writable path, also to bypass first of pip
				# attempt to copy file there
				try:
					shutil.copy(extracted_binary, path_dir)
					print("-"* 50)
					for i in range(3):
						print("|" + " " * 49 + "|")
					print("|   Installed to %s successfully." % path_dir)
					for i in range(3):
						print("|" + " " * 49 + "|")
					print("-"* 50)
					# run the binary!
					os.system("rcut -h")
					exit(0)
				except Exception as e:
					pass


			# if name.endswith('.zip'):

install_release()

