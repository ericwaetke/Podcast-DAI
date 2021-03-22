# Podcast Dynamic Ad Insertion


What kind of software is this?

Podcast-DAI is a Ad Inserter for your (self-hosted) podacast, programmed live on [Twitch](https://www.twitch.tv/ericwaetke) and in [Rust](https://www.rust-lang.org/).


## Todo:

- we have to use ffmpeg
	- save specificly cut mp3 w/o re-encoding
	- QUALITY OF LIFE: automatic download ffmpeg if it's not installed


- features
	- directory with all episodes 
	- varyable amount of cut points
		- fn cut_insert(cut_time, insertion_file)
	- we have to store the cut-points
	- use and convert into different formats (e.g .flac -> .mp3)