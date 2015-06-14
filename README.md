# pwtools
Permutation writing tools. This is a utility for generating permutations, either based on a term or just all possible permutations of a given length.

# Roadmap
Key:
* ~~Finished feature~~
* Unfinished/unimplemented feature

Planned features:
* ~~Permutations over a specified term~~
* ~~Permutations over a set of characters, for different ranges~~
* Multithreaded support
  * ~~Combinations~~
  * Permutations

# Performance
Below are two examples of timing the performance of multithreaded combination generation. The first one saves directly to /dev/null, and you can tell it is much faster to use four threads than to use one.

![/dev/null timing][dev-null-timing]

However, if you are actually writing to a hard disk (I am using a HDD, not an SSD), then you're going to run into the bottleneck of waiting on the OS to write to the hard drive. While the buffer may be filling just as quickly as it did with /dev/null, you're still waiting around for the kernel to process it.

![Real file on the hard drive timing][real-file-timing]

I don't have screenshots for these, but the same thing happens if you pipe it to a filter like wc, tee, sed, etc. Your output is going to show up only as fast as the slowest program in your pipeline.

[dev-null-timing]: https://github.com/alekratz/pwtools/raw/master/img/dev-null-timing.png
[real-file-timing]: https://github.com/alekratz/pwtools/raw/master/img/real-file-timing.png

# Disclaimer
Yes, this tool is specifically for permuting over a specified term. This is very similar to password generation. **However, you should not abuse this software. Do not use this software to break the law, including (but not limited to) hacking and breaking into systems without prior authorization to do so.**

This set of tools is for **educational** and **constructive** purposes only. This means that if you are a student of cryptography or cryptology and wanted to study tools and maybe complete a lab using these tools - you are more than welcome to do so (provided your lab allows you to use 3rd party tools). Likewise, if you are a cryptographer or cryptologist trying to assess the ability to break into a system - you are more than welcome to use this tool.

If you are planning to use this tool for other purposes (e.g. using this tool to pipe all of these permutations into an SSH session to try to crack a password), that is not permitted, and you are subject to any laws in your jurisdiction. You are responsible for how you use this tool. With great power comes great responsibility.

# License
GPL v3 Affero