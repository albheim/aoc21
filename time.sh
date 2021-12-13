#!/usr/bin/bash

for d in ./*/ ; do
    cd $d
    cargo build --release
    cd ..
done

echo "Running all"
start_time="$(date -u +%s.%N)"
for d in ./*/ ; do
    cd $d
    #echo "Running $(basename $d)"
    start_time1="$(date -u +%s.%N)"
    ./target/release/$(basename $d) a >> /dev/null
    ./target/release/$(basename $d) b >> /dev/null
    end_time1="$(date -u +%s.%N)"
    elapsed="$(bc <<<"$end_time1-$start_time1")"
    echo "$elapsed seconds"
    cd ..
done
end_time="$(date -u +%s.%N)"

elapsed="$(bc <<<"$end_time-$start_time")"
echo "Total of $elapsed seconds elapsed for process"