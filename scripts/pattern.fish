#!/usr/bin/env fish

set cmd $argv[1]
if test "$cmd" = all; or test "$cmd" = any; or test "$cmd" = ""
    for line in "$(curl https://conwaylife.com/patterns/ | rg .cells)"
        echo $line | string split '"' -f 2
    end
    return 0
end
set pattern "$(string replace '.cells' '' $cmd)"
echo "pattern: '$pattern'"
set url "https://conwaylife.com/patterns/$pattern.cells"
echo "url: '$url'"
if test "$argv[2]" = --download; or test "$argv[2]" = -d
    set p "/tmp/$pattern.cells"
    if test -e $p
        echo "already saved to '$p'"
    else
        echo "saving to '$p'"
        curl $url -o $p
    end
else
    curl $url | cgol-tui -
end
