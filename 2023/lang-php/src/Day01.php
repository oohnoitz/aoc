<?php

class Day01
{
    const MAPPING = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine'];

    public static function part1(string $input): int
    {
        $lines = preg_replace(subject: trim($input), pattern: '/([A-Za-z])/', replacement: '');
        $lines = explode(string: $lines, separator: \PHP_EOL);
        $output = 0;

        foreach ($lines as $line) {
            $f = substr($line, 0, 1);
            $l = substr($line, -1);

            $output += intval($f . $l);
        }

        return $output;
    }

    public static function part2(string $input): int
    {
        $lines = explode(string: trim($input), separator: \PHP_EOL);
        $output = 0;

        foreach ($lines as $line) {
            $length = strlen($line);
            $offset = 0;
            $f = null;
            $l = null;

            for ($offset = 0; $offset < $length; $offset++) {
                if (is_numeric($line[$offset])) {
                    $l = intval($line[$offset]);
                    $f = $f ?? $l;
                } else {
                    $string = substr($line, $offset);

                    foreach (self::MAPPING as $idx => $value) {
                        if (str_starts_with($string, $value)) {
                            $l = $idx + 1;
                            $f = $f ?? $l;
                        }
                    }
                }
            }

            $output += intval($f . $l);
        }

        return $output;
    }
}

$input = file_get_contents(filename: '../input/day01.txt');
echo sprintf("Day 01, Part 01 = %d", Day01::part1($input)) . \PHP_EOL;
echo sprintf("Day 01, Part 02 = %d", Day01::part2($input)) . \PHP_EOL;
