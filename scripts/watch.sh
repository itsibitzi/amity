inotifywait -m -r -e modify ./src | while read f
do
    echo "Detected changes, rebuilding..."
    /usr/bin/time -e 'Took %e seconds' ./scripts/build.sh
done