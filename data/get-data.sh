#!/usr/bin/env bash

set -e

youtube-dl --output testvideo.mp4 https://www.youtube.com/watch?v=aUdKzb4LGJI

download_url="https://github.com/nnsuite/testcases/raw/master/DeepLearningModels/tensorflow/ssdlite_mobilenet_v2"
wget ${download_url}/ssdlite_mobilenet_v2.pb
wget ${download_url}/coco_labels_list.txt

