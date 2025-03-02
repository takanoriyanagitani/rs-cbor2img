#!/bin/sh

export ENV_MAX_INPUT_SIZE=16777216

input=$( printf '\xa3aAa0aBa1aCa2' )

echo printing the original data using xxd...
printf '%s' $input | xxd
echo

echo printing a converted image file type...
printf '%s' $input | ./rs-cbor2img | file -
echo

echo printing a converted image using xxd...
printf '%s' $input | ./rs-cbor2img | xxd
echo

echo printing the original data using cbor2...
printf '%s' $input | python3 -m uv tool run cbor2
echo

echo printing a decoded data of the original data using xxd...
printf '%s' $input |
	./rs-cbor2img |
	ENV_DECODE=true ./rs-cbor2img |
	xxd
echo

echo printing a decoded data of the original data using cbor2...
printf '%s' $input |
	./rs-cbor2img |
	ENV_DECODE=true ./rs-cbor2img |
	python3 \
		-m uv \
		tool \
		run cbor2 \
		--sequence
echo Note: 'the last empty maps should be ignored(when padding required)'
