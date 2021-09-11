inotifywait -m -r -e modify ./src | while read f
do
    echo "Detected changes, rebuilding..."
    ./scripts/build.sh
done