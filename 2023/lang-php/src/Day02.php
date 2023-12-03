<?php

class Day02
{
    public static function part1(string $input): int
    {
        $lines = explode(string: trim($input), separator: \PHP_EOL);
        $output = 0;

        foreach ($lines as $line) {
            $line_split = explode(string: $line, separator: ':');
            $game_id = intval(substr(string: $line_split[0] ?? '', offset: 5));
            $invalid = false;

            $rounds = trim(string: $line_split[1] ?? '');
            $rounds = explode(string: $rounds, separator: ';');
            $rounds = array_map(array: $rounds, callback: fn ($round) => explode(string: trim($round), separator: ','));

            foreach ($rounds as $round) {
                $options = array_map(array: $round, callback: fn ($option) => explode(string: trim($option), separator: ' '));

                foreach ($options as $option) {
                    $count = intval($option[0]);
                    $color = $option[1];

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
            $line_split = explode(string: $line, separator: ':');
            $min_values = ['red' => 0, 'green' => 0, 'blue' => 0];

            $rounds = trim(string: $line_split[1] ?? '');
            $rounds = explode(string: $rounds, separator: ';');
            $rounds = array_map(array: $rounds, callback: fn ($round) => explode(string: trim($round), separator: ','));

            foreach ($rounds as $round) {
                $options = array_map(array: $round, callback: fn ($option) => explode(string: trim($option), separator: ' '));

                foreach ($options as $option) {
                    $count = intval($option[0]);
                    $color = $option[1];

                    $min_values[ $color ] = max($min_values[ $color ], intval($count));
                }
            }

            $output += ($min_values['red'] * $min_values['green'] * $min_values['blue']);
        }

        return $output;
    }
}

$input = file_get_contents(filename: '../input/day02.txt');
echo sprintf("Day 02, Part 01 = %d", Day02::part1($input)) . \PHP_EOL;
echo sprintf("Day 02, Part 02 = %d", Day02::part2($input)) . \PHP_EOL;
