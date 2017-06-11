KCOV_VERSION="33"
KCOV_INSTALL_PREFIX="${HOME}/kcov"

if [[ -f "$KCOV_INSTALL_PREFIX/bin/kcov" ]]; then
    $KCOV_INSTALL_PREFIX/bin/kcov --version
    exit 0
fi

curl -L https://github.com/SimonKagstrom/kcov/archive/v$KCOV_VERSION.tar.gz | tar -zxf -

pushd kcov-$KCOV_VERSION

mkdir build && pushd build

cmake -DCMAKE_INSTALL_PREFIX:PATH="${KCOV_INSTALL_PREFIX}" ..

make
make install

$KCOV_INSTALL_PREFIX/bin/kcov --version
popd
popd
