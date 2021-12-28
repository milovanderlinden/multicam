#!/bin/bash
# FFmpeg is an audio/video conversion tool
# ADDED RICO sleep allows pi to start services before grabbing videostreams
# for x line checks to see which videostreams are outputting data from usb
# -y Overwrite output files without asking
# -i input file url
# -nostdin To explicitly disable interaction you need to specify -nostdin (opposed to default -stdin)
# -loglevel quiet Show nothing at all; be silent
# -f segment Force input or output file format. The format is normally auto detected for input files and guessed from the file extension for output files, so this option is not needed in most cases
# -strftime Use strftime() on filename to expand the segment filename with localtime
# strftime is om de YMd naam te kunnen maken
# reset_timestamps at the beginning of each segment
# -segment_time Set segment duration to time, the value must be a duration specification
# -vcodec copy Set the video codec
# -acodec copy Set the audio codec
# vcodec en acodec copy kopieren een op een zonder de video te transformeren (sneller)
# ~/Videos.. naming convention

# sleep 1
# cd /home/pi
for x in /dev/video*; do
  m=$( echo $x | grep -oP "\\d+")
  ffmpeg -y \
    -i ${x} \
    -nostdin \
    -loglevel quiet \
    -f segment \
    -strftime 1 \
    -reset_timestamps 1 \
    -segment_time 60 \
    -vcodec copy \
    -acodec copy \
    ~/Videos/video${m}-%Y%m%d-%H%M%S.mp4 2> /dev/null &
done

--
dit is het script wat ik net gebruikte op de terug ingespoelde image.

#!/bin/bash
sleep 1
for x in /dev/video*; do
  m=$( echo $x | grep -oP "\\d+")
  ffmpeg -y \
    -i ${x} \
    -nostdin \
    -loglevel quiet \
    -f segment \
    -strftime 1 \
    -reset_timestamps 1 \
    -segment_time 30 \
    -vcodec copy \
    -acodec copy \
    ~/Videos/helmcam${m}-%Y%m%d-%H%M%S.mp4 2> /dev/null &
done
