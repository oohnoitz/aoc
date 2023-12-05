<?php

class Day04
{
    public static function part1(string $input): int
    {
        $lines = explode(string: trim($input), separator: \PHP_EOL);

        $cards = array_map(
            array: $lines,
            callback: function ($card) {
                [, $data] = explode(string: $card, separator: ':');

                [$numbers, $winning] = array_map(
                    array: explode(string: $data, separator: '|'),
                    callback: fn ($string) => preg_split('/\s+/', trim($string))
                );

                $match = count(array_intersect($numbers, $winning));
         
                if ($match > 0) {
                    return pow(2, $match - 1);
                } else {
                    return 0;
                }
            }
        );

        return array_sum($cards);
    }

    public static function part2(string $input): int
    {
        $lines = explode(string: trim($input), separator: \PHP_EOL);
        $cards = array_fill(start_index: 0, count: count($lines), value: 1);

        foreach ($lines as $game => $card) {
            [, $data] = explode(string: $card, separator: ':');

            [$numbers, $winning] = array_map(
                array: explode(string: $data, separator: '|'),
                callback: fn ($string) => preg_split('/\s+/', trim($string))
            );

            $match = count(array_intersect($numbers, $winning));
            $range = self::range(start: $game + 1, end: $game + $match);

            foreach ($range as $i) {
                $cards[$i] += $cards[$game];
            }
        }

        return array_sum($cards);
    }

    public static function range(int $start, int $end): array
    {
        $results = [];

        for ($i = $start; $i <= $end; $i++) {
             $results[] = $i;
        }

        return $results;
    }
}

$input = file_get_contents(filename: '../input/day04.txt');
echo sprintf("Day 04, Part 01 = %d", Day04::part1($input)) . \PHP_EOL;
echo sprintf("Day 04, Part 02 = %d", Day04::part2($input)) . \PHP_EOL;
