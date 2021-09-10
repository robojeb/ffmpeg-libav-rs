#!/bin/bash

mkdir -p tests
cd tests

if [ ! -f "./twentythousand.mp3" ]; then
    wget http://www.archive.org/download/twentythousandleagues_1311_librivox/twentythousandleagues_00_verne_128kb.mp3 -O ./twentythousand.mp3
fi


if [ ! -f "./twentythousand.flac" ]; then
    ffmpeg -i ./twentythousand.mp3 twentythousand.flac
fi

if [ ! -f "./t01.mp3" ]; then
    #avconv -i ./twentythousand.mp3 -af trim=69:120 t01.mp3
    ffmpeg -i ./twentythousand.mp3 -ss 00:01:09 -to 00:02:00 -c:v copy -c:a copy t01.mp3
fi

if [ ! -f "./t02.mp3" ]; then
    #avconv -i ./twentythousand.mp3 -af trim=69:120 t02.mp3
    ffmpeg -i ./twentythousand.mp3 -ss 00:02:00 -to 00:03:00 -c:v copy -c:a copy t02.mp3
fi

if [ ! -f "./t01.flac" ]; then
    #avconv -i ./twentythousand.mp3 -af trim=69:120 t01.mp3
    ffmpeg -i ./t01.mp3 t01.flac
fi

if [ ! -f "./t02.flac" ]; then
    #avconv -i ./twentythousand.mp3 -af trim=69:120 t02.mp3
    ffmpeg -i ./t02.mp3 t02.flac
fi
