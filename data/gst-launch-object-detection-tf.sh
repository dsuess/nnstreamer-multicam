#!/usr/bin/env bash

# For testing nnstreamer setup

gst-launch-1.0 \
    filesrc location=testvideo.mp4 ! qtdemux ! h264parse ! decodebin \
    ! videoscale ! videoconvert ! video/x-raw,width=320,height=180,format=RGB ! tee name=t \
    t. ! queue ! videoscale ! tensor_converter ! \
        tensor_filter framework=tensorflow model=ssdlite_mobilenet_v2.pb \
            input=3:640:360:1 inputname=image_tensor inputtype=uint8 \
            output=1,100:1,100:1,4:100:1 \
            outputname=num_detections,detection_classes,detection_scores,detection_boxes \
            outputtype=float32,float32,float32,float32 ! \
        tensor_decoder mode=bounding_boxes option1=tf-ssd option2=coco_labels_list.txt option4=320:180 option5=320:180 ! \
        compositor name=mix sink_0::zorder=2 sink_1::zorder=1 ! videoconvert ! \
        x264enc ! mp4mux ! filesink location=out.mp4 \
    t. ! queue ! mix.
