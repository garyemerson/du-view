# du-view

du-view converts the output of the du (disk usage) program to an html page that displays your files as a hierarchical and collapsible list that can be interacted with via arrow keys.

### Usage

See what's taking up space in your home directory:
```
du ~ | du-view > usage.html
```




Check usage on a remote machine, telling du to use a block size of 1024 bytes with the `-k` option and informing du-view to it should interpret the block size as 1024 bytes as well:
```
ssh bob@example.com 'du -k /' | du-view -k > usage.html
```

### Demo

![demo](demo.gif)
