for i in `find . -maxdepth 1 -mindepth 1 -type d`; do cd $i; cargo build; cd ..; done
