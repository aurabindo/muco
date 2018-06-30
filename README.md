# About

Music COpier is a lame tool for synchronising audio files across portable mass storage devices that may be plugged into different music playback systems like car stereo, bluetooth speaker, etc.

# Why?

Because I prefer to keep lossless formats of all audio in my laptop, which has the central library. My other players do not support flac sadly. Dear Honda, please add FLAC support to your car audio. I did not want to manually convert them and copy around. But this just happened to be a reason to learn some Rust.

So, the real reason, is to learn Rust! :D But I must admit, the code in the current state, sucks. Suggestions to improve are more than welcome!

# Usage

There are lots of unpolished areas, which I intent to improve. As of now, the
You first need to navigate to the original source of your music, called library
and initialize muco there.

```
muco library init
```

Now time to tell where muco will find your devices: 
```
muco device add --name Sasi --format mp3 flac --location /run/media/aj/AURA
```

* `name` is what you will call the device in muco
* `format` can be a list of supported formats. Currently mp3, flac, wma, aac are supported
* `location` is the path where it will be mounted on your system.

You can add any number of device like this.

Now all you need to do, is:
```
muco device sync
```

