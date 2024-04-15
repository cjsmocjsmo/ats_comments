for image in *.jpg; do
	ffmpeg -i "$image" -vf scale=200:150 "${image%.jpg}_thumb.webp";
	ffmpeg -i "$image" "${image%.jpg}.webp";
done
