# Execute `kcov` for every execute found in `target/debug/`.

KCOV_BIN="${HOME}/kcov/bin/kcov"

echo "Code coverage: ..."

test_exec=$(find target/debug -maxdepth 1 -type f -executable)

mapfile -t arr <<< "$test_exec"

for ex in $test_exec; do
    $KCOV_BIN --coveralls-id=$TRAVIS_JOB_ID --exclude-pattern=/.cargo,/usr/lib --verify target/cov "$ex"
done

echo " Done"
