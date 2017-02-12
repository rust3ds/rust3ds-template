#!/bin/bash

docker run -it --rm \
	--net=host \
	-v $(pwd):/root/code \
	rust_3ds \
   $*

