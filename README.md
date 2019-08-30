# About

MUsic COpier is a lame tool for synchronising audio files across portable mass storage devices that may be plugged into different music playback systems like car stereo, bluetooth speaker, etc.

# Why?

Because I prefer to keep lossless formats of all audio in my laptop, which is the main library. My other players do not support flac sadly. I did not want to manually convert them and copy around, and I like Rust, so here we are.

# Usage


You can have multiple libraries on a system. You can add one like so:

```
muco library add -n library_name -l /path/to/library
```

Now time to tell where muco will find your devices. You can only specify one supported format as of now: 
```
muco device add --name Sasi --format mp3 --location /run/media/aj/AURA
```

* `name` is what you will call the device in muco
* `format` can be a list of supported formats. Currently mp3 and flac transcoding are supported.
* `location` is the path where it will be mounted on your system.

You can add any number of device like this.

Now all you need to do, is:
```
muco sync
```

# Issues

Please use [GitLab](https://gitlab.com/aurabindo-public/muco.git) for issues
```
https://gitlab.com/aurabindo-public/muco.git
```

# Todo

At the moment this works only on Linux based OS. I have no intention of addding windows support, nor will I take patches for windows support. Stay away crapposoft!

