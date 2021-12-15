#!/usr/bin/bash

for d in ./*/ ; do
    cd $d
    cargo build --release
    cd ..
done

echo "Running individual"
for d in ./*/ ; do
    cd $d
    start_time="$(date -u +%s.%N)"
    ./target/release/$(basename $d) a >> /dev/null
    mid_time="$(date -u +%s.%N)"
    ./target/release/$(basename $d) b >> /dev/null
    end_time="$(date -u +%s.%N)"
    elapseda="$(bc <<<"$mid_time-$start_time")"
    elapsedb="$(bc <<<"$end_time-$mid_time")"
    echo "$(basename $d)a: $elapseda seconds"
    echo "$(basename $d)b: $elapsedb seconds"
    cd ..
done

echo "Running all"
start_time="$(date -u +%s.%N)"
for d in ./*/ ; do
    cd $d
    ./target/release/$(basename $d) a >> /dev/null
    ./target/release/$(basename $d) b >> /dev/null
    cd ..
done
end_time="$(date -u +%s.%N)"

elapsed="$(bc <<<"$end_time-$start_time")"
echo "Total: $elapsed seconds"