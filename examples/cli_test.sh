#!/bin/sh

gst-launch-1.0 videotestsrc ! rsstreamid stream_id=123 ! ximagesink
