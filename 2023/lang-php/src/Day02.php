<?php

class Day02
{
    public static function part1(string $input): int
    {
        $lines = explode(string: trim($input), separator: \PHP_EOL);
        $output = 0;

        foreach ($lines as $line) {
            [$part_1, $part_2] = explode(string: $line, separator: ':');
            $game_id = intval(substr(string: $part_1, offset: 5));
            $invalid = false;

            $rounds = self::parseRounds($part_2);

            foreach ($rounds as $round) {
                $options = array_map(array: $round, callback: fn ($option) => explode(string: trim($option), separator: ' '));

                foreach ($options as $option) {
                    [$count, $color] = $option;
                    $count = intval($count);

                    $invalid = match (true) {
                        $color === 'red' && $count > 12 => true,
                        $color === 'green' && $count > 13 => true,
                        $color === 'blue' && $count > 14 => true,
                        default => $invalid,
                    };
                }
            }

            if ($invalid === false) {
                $output += $game_id;
            }
        }

        return $output;
    }

    public static function part2(string $input): int
    {
        $lines = explode(string: trim($input), separator: \PHP_EOL);
        $output = 0;

        foreach ($lines as $line) {
            [, $part_2] = explode(string: $line, separator: ':');
            $min_values = ['red' => 0, 'green' => 0, 'blue' => 0];

            $rounds = self::parseRounds($part_2);

            foreach ($rounds as $round) {
                $options = array_map(array: $round, callback: fn ($option) => explode(string: trim($option), separator: ' '));

                foreach ($options as $option) {
                    [$count, $color] = $option;
                    $count = intval($option[0]);

                    $min_values[ $color ] = max($min_values[ $color ], intval($count));
                }
            }

            ['red' => $r, 'green' => $g, 'blue' => $b] = $min_values;
            $output += ($r * $g * $b);
        }

        return $output;
    }

    public static function parseRounds(string $input): array
    {
        $rounds = trim(string: $input);
        $rounds = explode(string: $rounds, separator: ';');

        return array_map(array: $rounds, callback: fn ($round) => explode(string: trim($round), separator: ','));
    }
}

$input = file_get_contents(filename: '../input/day02.txt');
echo sprintf("Day 02, Part 01 = %d", Day02::part1($input)) . \PHP_EOL;
echo sprintf("Day 02, Part 02 = %d", Day02::part2($input)) . \PHP_EOL;
