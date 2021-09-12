start_time="$(date -u +%s)"

inotifywait -m -r -e modify ./src --format '%f' | while read f
do
    now="$(date -u +%s)"
    elapsed="$((now-start_time))"
    if (( elapsed > 5 )); then
        echo "Detected changes in $f, rebuilding..."
        /usr/bin/time -f 'Took %e seconds' ./scripts/build.sh
        start_time="$now"
    fi
done